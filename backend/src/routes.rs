use crate::api_structs::{AiarenaApiBots, Bots, Maps};
use crate::bots::start_bot;
use crate::paths::{find_available_bots, find_available_maps};
use crate::results_data::{save_to_file, FileResultsData, GameResult, ResultsData, RESULTS_FILE};
use crate::run_game_data::RunGameData;
use crate::settings_data::{settings_file_exists, settings_okay, SettingsFormData};
use crate::supervisor::{Supervisor, SupervisorChannel};
use actix_web::client::Client;
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};
pub use actix_web::{App, HttpResponse, HttpServer, Result};
use handlebars::Handlebars;
use log::error;
use paperclip::actix::api_v2_operation;
use paperclip::actix::web::{Bytes, Form, Json};
use rand::prelude::IteratorRandom;
use rust_ac::server::RustServer;
use std::fs::File;
use std::thread::JoinHandle;

const AIARENA_URL: &str = "https://aiarena.net";
static mut SUPERVISOR_CHANNEL: Option<SupervisorChannel> = None;
static mut RUST_SERVER_HANDLE: Option<JoinHandle<()>> = None;
pub const CLIENT_PORT: i32 = 8642;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub async fn index(hb: actix_web::web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "settings_okay": settings_okay() });
    if settings_file_exists() {
        let body = hb.render("index", &data).unwrap();

        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Found()
            .header("Location", "/settings")
            .finish()
    }

    // Ok(HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../static/index.html")))
}
#[allow(clippy::needless_return)]
#[api_v2_operation]
pub async fn run_games(run_game_data: Bytes) -> Result<HttpResponse> {
    let b = String::from_utf8(run_game_data.to_vec()).unwrap();
    let run_game_data: RunGameData = serde_json::from_str(&b)?;

    unsafe {
        if RUST_SERVER_HANDLE.is_none() {
            RUST_SERVER_HANDLE =
                Some(RustServer::new(format!("127.0.0.1:{}", CLIENT_PORT).as_str()).run());
            // SUPERVISOR_CHANNEL =
            //     Some(Supervisor::connect().map_err(|e| ErrorInternalServerError(e.to_string()))?);
        }
    }
    let settings_data = SettingsFormData::load_from_file()?;
    let maps = find_available_maps();
    std::thread::spawn(move || {
        for _ in 0..run_game_data.iterations {
            let mut channel = Supervisor::connect().unwrap();
            // unsafe {
            //     SUPERVISOR_CHANNEL = Supervisor::connect()
            //         .map_err(|e| ErrorInternalServerError(e.to_string()))
            //         .ok();
            // }
            for mut map in &run_game_data.map {
                if map == "Random" {
                    map = maps.iter().choose(&mut rand::thread_rng()).unwrap();
                }
                for bot1 in &run_game_data.bot1 {
                    for bot2 in &run_game_data.bot2 {
                        let mut config = rust_ac::config::Config::new();
                        let max_match_id = FileResultsData::load_from_file()
                            .unwrap_or_default()
                            .max_match_id();
                        config.map = map.clone();
                        config.disable_debug = !matches!(settings_data.allow_debug.as_str(), "On");
                        config.match_id = max_match_id;
                        config.player1 = bot1.clone();
                        config.player2 = bot2.clone();
                        config.real_time = run_game_data.realtime;
                        config.visualize = run_game_data.visualize;
                        config.max_game_time = settings_data.max_game_time as u32;
                        let str_config = serde_json::to_string(&config).unwrap();
                        // unsafe {
                        //     let channel = SUPERVISOR_CHANNEL.as_mut().unwrap();
                        let _rec = channel.recv();
                        channel
                            .send(str_config)
                            .map_err(|e| {
                                println!("1{:?}", e.to_string());
                                return ErrorInternalServerError(e.to_string());
                            })
                            .unwrap();

                        let _rec = channel.recv();
                        let _c = start_bot(
                            bot1.clone(),
                            settings_data.bot_directory_location.clone(),
                            "".to_string(),
                        );
                        let _rec = channel.recv();
                        let _c2 = start_bot(
                            bot2.clone(),
                            settings_data.bot_directory_location.clone(),
                            "".to_string(),
                        );
                        let mut results_data: Option<ResultsData> = None;
                        for msg in channel.iter() {
                            println!("{:?}", msg.clone());
                            if msg.contains("MatchID") {
                                println!("\n\n\n\nOk");
                                results_data = serde_json::from_str(&msg).ok();
                                break;
                            }
                        }
                        if let Some(data) = results_data {
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

                                    if let Err(e) = save_to_file(&frd, RESULTS_FILE) {
                                        error!("{}", e.to_string());
                                    }
                                }
                            };
                        }
                        // }
                        channel.send("Disconnect".to_string());
                    }
                }
            }
        }
    });

    Ok(HttpResponse::Ok().finish())
}

pub async fn settings(hb: actix_web::web::Data<Handlebars<'_>>) -> HttpResponse {
    let context = json!({});
    let body = hb.render("settings", &context).unwrap();
    HttpResponse::Ok().body(body)
    // Ok(HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../static/settings.html")))
}

pub async fn watch(hb: actix_web::web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("watch", &data).unwrap();
    HttpResponse::Ok().body(body)
    // Ok(HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../static/index.html")))
}

#[api_v2_operation]
pub async fn get_maps() -> Json<Maps> {
    Json(Maps {
        maps: find_available_maps(),
    })
}
#[api_v2_operation]
pub async fn get_bots() -> Json<Bots> {
    Json(Bots {
        bots: find_available_bots(),
    })
}
#[api_v2_operation]
pub async fn handle_data(form: Form<SettingsFormData>) -> Result<HttpResponse> {
    match form.save_to_file() {
        Ok(_) => Ok(HttpResponse::Found().header("Location", "/").finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e.to_string())),
    }
}
#[api_v2_operation]
pub async fn get_arena_bots() -> Result<Json<AiarenaApiBots>> {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        let client = Client::default();
        let mut response = client
            .get(format!(
                "{}{}",
                AIARENA_URL, r#"/api/bots/?&format=json&bot_zip_publicly_downloadable=true"#
            )) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .header(
                "Authorization",
                format!("Token  {}", settings_data.api_token),
            )
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
#[api_v2_operation]
pub async fn get_settings() -> Result<Json<SettingsFormData>> {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        Ok(Json(settings_data))
    } else {
        let settings_data = SettingsFormData::default();
        Ok(Json(settings_data))
    }
}

#[api_v2_operation]
pub async fn get_results() -> Result<Json<FileResultsData>> {
    if let Ok(settings_data) = FileResultsData::load_from_file() {
        Ok(Json(settings_data))
    } else {
        let settings_data = FileResultsData::default();
        Ok(Json(settings_data))
    }
}
#[api_v2_operation]
pub async fn clear_results() -> Result<HttpResponse> {
    let results_file = File::open(RESULTS_FILE)?;
    results_file.set_len(0)?;
    Ok(HttpResponse::Ok().finish())
}
