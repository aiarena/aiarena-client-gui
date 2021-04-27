use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use actix_web::error::ErrorInternalServerError;
use actix_web::Result;
use directories::ProjectDirs;
use log::error;
use std::env::var_os;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

static SETTINGS_FILE: &str = "settings.json";

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct SettingsFormData {
    #[serde(default)]
    pub bot_directory_location: String,
    #[serde(default)]
    pub sc2_directory_location: String,
    #[serde(default)]
    pub replay_directory_location: String,
    #[serde(default, rename = "API_token")]
    pub api_token: String,
    #[serde(default = "default_max_game_time")]
    pub max_game_time: u64,
    #[serde(default)]
    pub allow_debug: bool,
}
impl SettingsFormData {
    pub fn settings_file() -> Result<PathBuf> {
        let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
            .ok_or_else(|| ErrorInternalServerError("Could not create Project Directory"))?;
        if !project_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(project_dirs.data_local_dir())?;
        }
        Ok(project_dirs.data_local_dir().join(&SETTINGS_FILE))
    }
    pub fn load_from_file() -> Result<Self> {
        let mut f: File;
        let settings_file = Self::settings_file()?;
        if !settings_file.exists() {
            f = File::create(settings_file)?;
        } else {
            f = File::open(settings_file)?;
        }
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        // Deserialize and print Rust data structure.
        let mut data: SettingsFormData = serde_json::from_str(&contents)?;
        if data.sc2_directory_location.is_empty() {
            data.sc2_directory_location = match var_os("SC2_PROXY_BASE") {
                Some(x) => Path::new(&x).display().to_string(),
                None => match var_os("SC2PATH") {
                    Some(x) => Path::new(&x).display().to_string(),
                    None => "".to_string(),
                },
            };
            if let Err(e) = data.save_to_file() {
                error!("{}", e.to_string());
            }
        }

        Ok(data)
    }
    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut f: File;
        let settings_file = Self::settings_file()?;
        if !settings_file.exists() {
            f = File::create(settings_file)?;
        } else {
            f = OpenOptions::new().write(true).open(settings_file)?;
        }
        // Clear file
        f.set_len(0)?;
        f.write_all((serde_json::to_string_pretty(&self)?).as_bytes())?;
        Ok(())
    }
}
pub fn settings_okay() -> bool {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        return !(settings_data.bot_directory_location.is_empty()
            || settings_data.sc2_directory_location.is_empty()
            || settings_data.replay_directory_location.is_empty());
    }
    false
}
pub fn settings_file_exists() -> bool {
    if let Ok(settings_file) = SettingsFormData::settings_file() {
        settings_file.exists()
    } else {
        false
    }
}
fn default_max_game_time() -> u64 {
    60480
}
