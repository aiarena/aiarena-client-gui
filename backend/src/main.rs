mod api_structs;
mod paths;
mod results_data;
mod run_game_data;
mod settings_data;
mod supervisor;
pub mod routes;
pub mod bots;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde;


use crate::api_structs::{AiarenaApiBots, Bots, Maps};
use crate::paths::{find_available_bots, find_available_maps};
use crate::run_game_data::RunGameData;
use crate::settings_data::{settings_file_exists, settings_okay, SettingsFormData};
use crate::supervisor::Supervisor;
use actix_web::client::{Client, Connector};
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};
pub use actix_web::{App, HttpResponse, HttpServer, Result};
use crossbeam::{self, channel::Sender};
use handlebars::Handlebars;
use openssl::ssl::{SslConnector, SslMethod};
use paperclip::actix::web::{Bytes, Form, Json};
use paperclip::actix::{api_v2_operation, web, OpenApiExt};
use rand::prelude::IteratorRandom;
use rust_ac::server::RustServer;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use crate::routes::*;


const AIARENA_URL: &str = "https://aiarena.net";
static mut SUPERVISOR_SENDER: Option<Sender<String>> = None;
static mut RUST_SERVER_HANDLE: Option<JoinHandle<()>> = None;
pub const CLIENT_PORT: i32 = 8642;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "rust_ac=trace");
    env_logger::init();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(handlebars_ref.clone())
            .service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
            .service(actix_web::web::resource("/watch").route(actix_web::web::get().to(watch)))
            .service(
                actix_web::web::resource("/settings").route(actix_web::web::get().to(settings)),
            )
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .wrap_api()
            .service(web::resource("/get_maps").route(web::get().to(get_maps)))
            .service(web::resource("/get_bots").route(web::get().to(get_bots)))
            .service(web::resource("/get_settings").route(web::get().to(get_settings)))
            .service(web::resource("/get_arena_bots").route(web::get().to(get_arena_bots)))
            .service(web::resource("/run_games").route(web::post().to(run_games)))
            .service(web::resource("/handle_data").route(web::post().to(handle_data)))
            .with_json_spec_at("/api")
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
