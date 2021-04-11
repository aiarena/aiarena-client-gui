use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use actix_web::Result;
use log::error;
use std::env::var_os;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

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
    pub allow_debug: String,
}
impl SettingsFormData {
    pub fn load_from_file() -> Result<Self> {
        let mut f: File;
        if !Path::new(&SETTINGS_FILE).exists() {
            f = File::create(&SETTINGS_FILE)?;
        } else {
            f = File::open(&SETTINGS_FILE)?;
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
        if !Path::new(&SETTINGS_FILE).exists() {
            f = File::create(&SETTINGS_FILE)?;
        } else {
            f = OpenOptions::new().write(true).open(&SETTINGS_FILE)?;
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
    Path::new(&SETTINGS_FILE).exists()
}
fn default_max_game_time() -> u64 {
    60480
}
