mod api_structs;
mod paths;
mod results_data;
mod run_game_data;
mod settings_data;
mod supervisor;
pub mod routes;
pub use actix_web::HttpServer;
pub use actix_web_static_files;
pub use handlebars::Handlebars;
pub use actix_web;
pub use paperclip::actix::{web, OpenApiExt};
pub use actix;


#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde;


