use crate::server::routes::*;
use actix_cors::Cors;

use actix_web::{dev::Server, web, App, HttpServer};

pub mod api_structs;
pub mod routes;

pub fn get_server() -> std::io::Result<Server> {
    Ok(HttpServer::new(move || {
        App::new()
            .service(web::resource("/get_maps").route(web::get().to(get_maps)))
            .service(web::resource("/get_bots").route(web::get().to(get_bots)))
            .service(web::resource("/get_settings").route(web::get().to(get_settings)))
            .service(web::resource("/get_arena_bots").route(web::get().to(get_arena_bots)))
            .service(web::resource("/run_games").route(web::post().to(run_games)))
            .service(web::resource("/handle_data").route(web::post().to(handle_data)))
            .service(web::resource("/get_results").route(web::get().to(get_results)))
            .service(web::resource("/clear_results").route(web::post().to(clear_results)))
            .wrap(Cors::permissive())
    })
    .bind("127.0.0.1:8082")?
    .run())
}
