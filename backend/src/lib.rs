mod api_structs;
pub mod bots;
mod paths;
mod results_data;
pub mod routes;
mod run_game_data;
pub mod server;
mod settings_data;
mod supervisor;

pub use actix;
pub use actix_web;
pub use actix_web::HttpServer;
pub use actix_web_static_files;
pub use handlebars::Handlebars;
pub use paperclip::actix::{web, OpenApiExt};

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde;
