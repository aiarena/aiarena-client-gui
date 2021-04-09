mod api_structs;
pub mod bots;
mod paths;
mod results_data;
pub mod routes;
mod run_game_data;
mod settings_data;
mod supervisor;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde;

use crate::routes::*;
pub use actix_web::{App, HttpResponse, HttpServer, Result};
use handlebars::Handlebars;
use paperclip::actix::{web, OpenApiExt};

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
            // .service(web::resource("/get_arena_bots").route(web::get().to(get_arena_bots)))
            .service(web::resource("/run_games").route(web::post().to(run_games)))
            .service(web::resource("/handle_data").route(web::post().to(handle_data)))
            .with_json_spec_at("/api")
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
