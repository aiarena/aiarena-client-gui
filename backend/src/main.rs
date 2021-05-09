mod bot;
mod errors;
pub mod files;
mod helpers;
mod json_structs;
mod macros;
mod paths;
pub mod server;
mod supervisor;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate anyhow;

pub use actix_web::{App, HttpResponse, HttpServer, Result};
use aiarena_client_gui_backend_lib::server::get_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    get_server()?.await
}
