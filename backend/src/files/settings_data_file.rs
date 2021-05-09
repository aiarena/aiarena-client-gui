use serde::{Deserialize, Serialize};

use crate::{errors::MyError, json_structs::file_json::JSONFile};

use anyhow::Result;
use directories::ProjectDirs;
use fs2::FileExt;
use log::error;
use std::{
    env::var_os,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

static SETTINGS_FILE: &str = "settings.json";

#[derive(Deserialize, Serialize, Debug, Default)]
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
impl SettingsFormData {}

impl JSONFile for SettingsFormData {
    fn file_path() -> Result<PathBuf, MyError> {
        let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
            .ok_or_else(|| anyhow!("Could not create Project Directory"))?;
        if !project_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(project_dirs.data_local_dir())?;
        }
        Ok(project_dirs.data_local_dir().join(&SETTINGS_FILE))
    }

    fn load_from_file() -> Result<Self, MyError> {
        let mut f: File;
        let settings_file = Self::file_path()?;
        if !settings_file.exists() {
            f = File::create(settings_file)?;
        } else {
            f = File::open(settings_file)?;
        }
        let mut contents = String::new();
        f.lock_exclusive()?;
        f.read_to_string(&mut contents)?;
        f.unlock()?;
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
}
#[allow(dead_code)]
pub fn settings_okay() -> bool {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        return !(settings_data.bot_directory_location.is_empty()
            || settings_data.sc2_directory_location.is_empty()
            || settings_data.replay_directory_location.is_empty());
    }
    false
}
#[allow(dead_code)]
pub fn settings_file_exists() -> bool {
    if let Ok(settings_file) = SettingsFormData::file_path() {
        settings_file.exists()
    } else {
        false
    }
}
fn default_max_game_time() -> u64 {
    60480
}
