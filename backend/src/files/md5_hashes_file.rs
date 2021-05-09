use crate::{errors::MyError, json_structs::file_json::JSONFile};
use anyhow::Result;
use directories::ProjectDirs;
use std::{collections::HashMap, path::PathBuf};

static MD5_HASHES_FILE: &str = "bot_hashes.json";

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MD5HashesFile {
    bots: HashMap<String, BotHash>,
}
impl MD5HashesFile {
    pub fn get_hash(&self, bot: &str) -> Option<&BotHash> {
        self.bots.get(bot)
    }
    pub fn get_hash_mut(&mut self, bot: &str) -> Option<&mut BotHash> {
        self.bots.get_mut(bot)
    }
    pub fn update_zip_hash(&mut self, bot: &str, hash: &str) {
        match self.get_hash_mut(bot) {
            None => {
                let empty_hash = BotHash {
                    bot_zip_md5hash: Some(hash.to_string()),
                    ..Default::default()
                };
                self.bots.insert(bot.to_string(), empty_hash);
            }
            Some(x) => x.bot_zip_md5hash = Some(hash.to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BotHash {
    bot_zip_md5hash: Option<String>,
    bot_data_md5hash: Option<String>,
}

impl BotHash {
    pub fn get_zip_hash(&self) -> Option<&String> {
        self.bot_zip_md5hash.as_ref()
    }
    pub fn get_data_hash(&self) -> Option<&String> {
        self.bot_data_md5hash.as_ref()
    }
}
// impl DBStruct

impl JSONFile for MD5HashesFile {
    fn file_path() -> Result<PathBuf, MyError> {
        let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
            .ok_or_else(|| MyError::new("Could not create Project Directory"))?;
        if !project_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(project_dirs.data_local_dir())?;
        }
        Ok(project_dirs.data_local_dir().join(&MD5_HASHES_FILE))
    }
}
