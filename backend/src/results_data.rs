use serde::{Deserialize, Serialize};

use actix_web::Result;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

static RESULTS_FILE: &str = "results.json";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResultsData {
    pub bot_directory_location: String,
    pub sc2_directory_location: String,
    pub replay_directory_location: String,
    pub api_token: String,
    pub max_game_time: u64,
    pub allow_debug: String,
}
impl ResultsData {
    pub fn load_from_file() -> Result<Self, Box<dyn Error>> {
        let mut f: File;
        if !Path::new(&RESULTS_FILE).exists() {
            f = File::create(&RESULTS_FILE)?;
        } else {
            f = File::open(&RESULTS_FILE)?;
        }
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        // Deserialize and print Rust data structure.
        let data: ResultsData = serde_json::from_str(&contents)?;

        Ok(data)
    }
    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut f: File;
        if !Path::new(&RESULTS_FILE).exists() {
            f = File::create(&RESULTS_FILE)?;
        } else {
            f = OpenOptions::new().write(true).open(&RESULTS_FILE)?;
        }
        // Clear file
        f.set_len(0)?;
        f.write_all((serde_json::to_string_pretty(&self)?).as_bytes())?;
        Ok(())
    }
}
