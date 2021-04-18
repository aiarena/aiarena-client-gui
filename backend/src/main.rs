mod api_structs;
pub mod bots;
mod paths;
mod results_data;
pub mod routes;
mod run_game_data;
pub mod server;
mod settings_data;
mod supervisor;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

pub use actix_web::{App, HttpResponse, HttpServer, Result};
use aiarena_client_gui_backend_lib::server::get_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    get_server("./static/")?.await
}
