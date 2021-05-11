use crate::{
    bot::{aiarena_bots::BotDownloadClient, start_bot},
    files::{results_data_file::FileResultsData, settings_data_file::SettingsFormData},
    json_structs::{
        file_json::JSONFile,
        results_data::{GameResult, ResultsData},
        run_game_data::RunGameData,
    },
    paths::{find_available_bots, find_available_maps},
    server::api_structs::{AiarenaApiBots, Bots, Maps},
    supervisor::Supervisor,
};

use actix_web::{
    client::{Client, Connector},
    error::{ErrorBadGateway, ErrorInternalServerError},
    web::{Bytes, Form, Json},
    HttpResponse, Result,
};
use itertools::Itertools;
use log::{debug, error, info};
use rand::prelude::IteratorRandom;
use rust_ac::server::RustServer;
use serde_json::Value;
use std::fs::OpenOptions;

use crate::errors::MyError;

use std::thread::JoinHandle;
use std::time::Duration;

pub const AIARENA_URL: &str = "https://aiarena.net";
static mut RUST_SERVER_HANDLE: Option<JoinHandle<()>> = None;
pub const CLIENT_PORT: i32 = 8642;

pub async fn run_games(run_game_data: Bytes) -> Result<HttpResponse> {
    let b = String::from_utf8(run_game_data.to_vec()).unwrap();
    let run_game_data: RunGameData = serde_json::from_str(&b)?;

    unsafe {
        if RUST_SERVER_HANDLE.is_none() {
            RUST_SERVER_HANDLE =
                Some(RustServer::new(format!("127.0.0.1:{}", CLIENT_PORT).as_str()).run());
        }
    }
    let settings_data =
        SettingsFormData::load_from_file().map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let maps = find_available_maps();

    let bot_client = BotDownloadClient::new();
    let bots: Vec<String> = run_game_data
        .bot1
        .clone()
        .iter()
        .chain(run_game_data.bot2.clone().iter())
        .dedup()
        .filter(|x| x.contains(" (AI-Arena)"))
        .cloned()
        .collect();

    // bots.append(&mut run_game_data.bot2.clone());
    // bots.sort();
    // bots.dedup();
    // for bot in bots.iter()
    // //     .clone()
    // //     .iter()
    // //     .chain(run_game_data.bot2.clone().iter())
    // //     .dedup()
    // {
    //     if bot.contains(" (AI-Arena)") {
    //         futures.push(bot_client.get_and_download_bot_zip(bot.clone()));
    //     }
    // }
    // let r = futures::future::join_all(futures).await;
    // for (result, bot) in r.iter().zip(bots) {
    //     info!("{:?} - Download {:?}", bot, result);
    // }
    for (bot, result) in bot_client
        .get_and_download_all(&bots)
        .await
        .map_err(MyError::from)?
    {
        info!("{:?} - Download {:?}", bot, result);
    }

    std::thread::spawn(move || {
        for _ in 0..run_game_data.iterations {
            for mut map in &run_game_data.map {
                if map == "Random" {
                    map = maps.iter().choose(&mut rand::thread_rng()).unwrap();
                }
                for bot1 in &run_game_data.bot1 {
                    for bot2 in &run_game_data.bot2 {
                        let mut channel = Supervisor::connect().unwrap();
                        let mut results_data: Option<ResultsData> = None;
                        let mut config = rust_ac::config::Config::new();
                        let max_match_id = FileResultsData::load_from_file()
                            .unwrap_or_default()
                            .max_match_id();
                        config.map = map.clone();
                        config.disable_debug = !settings_data.allow_debug;
                        config.match_id = max_match_id;
                        config.player1 = bot1.clone();
                        config.player2 = bot2.clone();
                        config.real_time = run_game_data.realtime;
                        config.visualize = run_game_data.visualize;
                        config.max_game_time = settings_data.max_game_time as u32;
                        let str_config = serde_json::to_string(&config).unwrap();
                        let _rec = channel.recv();
                        channel
                            .send(str_config)
                            .map_err(|e| {
                                error!("{:?}", e.to_string());
                                ErrorInternalServerError(e.to_string())
                            })
                            .unwrap();

                        let _rec = channel.recv();
                        let _c = start_bot(
                            bot1.clone(),
                            settings_data.bot_directory_location.clone(),
                            "".to_string(),
                        );
                        let mut bots: [bool; 2] = [false; 2];
                        match channel.recv_timeout(10) {
                            Ok(result) => {
                                let v: Value = serde_json::from_str(&result).unwrap();
                                if v["Bot"] == "Connected" {
                                    bots[0] = true;
                                }
                            }
                            Err(_) => {
                                results_data = Some(ResultsData::init_error(&config));
                            }
                        }
                        if bots[0] {
                            let _c2 = start_bot(
                                bot2.clone(),
                                settings_data.bot_directory_location.clone(),
                                "".to_string(),
                            );
                            match channel.recv_timeout(10) {
                                Ok(result) => {
                                    let v: Value = serde_json::from_str(&result).unwrap();
                                    if v["Bot"] == "Connected" {
                                        bots[1] = true;
                                    }
                                }
                                Err(_) => {
                                    debug!("Init error. Resetting Supervisor");
                                    results_data = Some(ResultsData::init_error(&config));
                                    channel.send("Reset".to_string()).unwrap();
                                    let confirmation = channel.recv();
                                    debug!("{:?}", confirmation);
                                }
                            }
                        }
                        if results_data.is_none() {
                            for msg in channel.iter() {
                                debug!("{:?}", msg.clone());
                                if msg.contains("MatchID") {
                                    debug!("\n\n\n\nOk");
                                    results_data = serde_json::from_str(&msg).ok();
                                    break;
                                }
                            }
                        }
                        if let Some(data) = results_data {
                            debug!("Found results data");
                            let game_result: GameResult = data.into();
                            match FileResultsData::load_from_file() {
                                Ok(mut x) => {
                                    x.add_result(game_result.clone());
                                    if let Err(e) = x.save_to_file() {
                                        error!("{}", e.to_string());
                                    }
                                }
                                Err(_) => {
                                    let mut frd = FileResultsData::default();
                                    frd.add_result(game_result);

                                    if let Err(e) = frd.save_to_file() {
                                        error!("{}", e.to_string());
                                    }
                                }
                            };
                        }
                        if let Err(e) = channel.send("Disconnect".to_string()) {
                            error!("{:?}", e.to_string());
                        }

                        debug!("Finished");
                    }
                }
            }
        }
    });

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_maps() -> Json<Maps> {
    Json(Maps {
        maps: find_available_maps(),
    })
}

pub async fn get_bots() -> Json<Bots> {
    Json(Bots {
        bots: find_available_bots(),
    })
}

pub async fn handle_data(form: Form<SettingsFormData>) -> Result<HttpResponse> {
    match form.save_to_file() {
        Ok(_) => Ok(HttpResponse::Ok().body("Success".to_string())),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e.to_string())),
    }
}
#[cfg(test)]
pub async fn get_arena_bots_env() -> Result<Json<AiarenaApiBots>> {
    let api_token =
        std::env::var_os("AIARENATOKEN").ok_or(ErrorBadGateway("Ai Arena Token not found"))?;
    let connector = Connector::new().timeout(Duration::from_secs(60)).finish();
    let client = Client::builder()
        .connector(connector)
        .timeout(Duration::from_secs(60))
        .finish();

    let mut response = client
        .get(format!(
            "{}{}",
            AIARENA_URL,
            r#"/api/bots/?&format=json&bot_zip_publicly_downloadable=true&ordering=name"#
        )) // <- Create request builder
        .header("User-Agent", "Actix-web")
        .header(
            "Authorization",
            format!("Token  {}", api_token.to_str().unwrap().to_string()),
        )
        .timeout(Duration::from_secs(60))
        .send() // <- Send https request
        .await?;

    let body = response.body().await?;
    let s =
        String::from_utf8(body.to_vec()).map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let aiarena_api_bots: AiarenaApiBots = serde_json::from_str(&s)?;
    return Ok(Json(aiarena_api_bots));
}
pub async fn get_arena_bots() -> Result<Json<AiarenaApiBots>> {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        let connector = Connector::new().timeout(Duration::from_secs(60)).finish();
        let client = Client::builder()
            .connector(connector)
            .timeout(Duration::from_secs(60))
            .finish();
        let mut response = client
            .get(format!(
                "{}{}",
                AIARENA_URL,
                r#"/api/bots/?&format=json&bot_zip_publicly_downloadable=true&ordering=name"#
            )) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .header(
                "Authorization",
                format!("Token  {}", settings_data.api_token),
            )
            .timeout(Duration::from_secs(60))
            .send() // <- Send https request
            .await?;

        let body = response.body().await?;
        let s = String::from_utf8(body.to_vec())
            .map_err(|e| ErrorInternalServerError(e.to_string()))?;
        let aiarena_api_bots: AiarenaApiBots = serde_json::from_str(&s)?;
        return Ok(Json(aiarena_api_bots));
    }
    Err(ErrorBadGateway("Could not connect to AiArena API"))
}
#[allow(dead_code)]
async fn test(bytes: Bytes) -> HttpResponse {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => {
            println!("Hello, {}!\n", text);
        }
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    HttpResponse::Ok().finish()
}

pub async fn get_settings() -> Result<Json<SettingsFormData>> {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        Ok(Json(settings_data))
    } else {
        let settings_data = SettingsFormData::default();
        Ok(Json(settings_data))
    }
}

pub async fn get_results() -> Result<Json<FileResultsData>> {
    if let Ok(results_data) = FileResultsData::load_from_file() {
        Ok(Json(results_data))
    } else {
        let results_data = FileResultsData::default();
        Ok(Json(results_data))
    }
}

pub async fn clear_results() -> Result<HttpResponse> {
    let results_file_path = FileResultsData::file_path()?;
    let results_file = OpenOptions::new().write(true).open(results_file_path)?;
    results_file.set_len(0)?;
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_get_arenabots() {
        let e = get_arena_bots_env().await;
        if e.is_err() {
            println!("{:?}", e);
        }
        assert!(e.is_ok())
    }
}
