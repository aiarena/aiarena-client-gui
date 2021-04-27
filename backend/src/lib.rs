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
use directories::ProjectDirs;
pub use handlebars::Handlebars;
pub use paperclip::actix::{web, OpenApiExt};

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde;

pub fn project_directory() -> Result<String, Box<dyn std::error::Error>> {
    let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
        .ok_or_else(|| "Could not find Project Directory".to_string())?;
    if !project_dirs.data_local_dir().exists() {
        std::fs::create_dir_all(project_dirs.data_local_dir())?;
    }
    Ok(project_dirs.data_local_dir().display().to_string())
}
