use actix_web::Result;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub static RESULTS_FILE: &str = "results.json";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResultsData {
    #[serde(default, rename = "Result")]
    results: HashMap<String, String>,
    #[serde(default, rename = "GameTime")]
    game_time: u64,
    #[serde(default, rename = "GameTimeSeconds")]
    game_time_seconds: f64,
    #[serde(default, rename = "AverageFrameTime")]
    average_frame_time: HashMap<String, Option<f64>>,
    #[serde(default, rename = "Status")]
    status: String,
    #[serde(default, rename = "Bots")]
    bots: HashMap<u8, String>,
    #[serde(default, rename = "Map")]
    map: String,
    #[serde(default, rename = "ReplayPath")]
    replay_path: String,
    #[serde(default, rename = "MatchID")]
    match_id: i64,
}
impl ResultsData {
    pub fn get_bot1(&self) -> String {
        self.bots.get(&1).unwrap().clone()
    }
    pub fn get_bot2(&self) -> String {
        self.bots.get(&2).unwrap().clone()
    }
    pub fn get_winner_result(&self) -> (String, String) {
        let p1 = self.get_bot1();
        let p2 = self.get_bot2();
        if self.results.values().any(|x| x == "SC2Crash") {
            ("".to_string(), "Error".to_string())
        } else {
            let p1_result;
            let p2_result;
            p1_result = match self.results.get(&p1).unwrap().as_str() {
                "Crash" => Some((p2.as_ref(), "Player1Crash")),
                "Timeout" => Some((p2.as_ref(), "Player1TimeOut")),
                "Victory" => Some((p1.as_ref(), "Player1Win")),
                "Defeat" => Some((p2.as_ref(), "Player2Win")),
                "Tie" => Some(("Tie", "Tie")),
                "InitializationError" => Some(("", "InitializationError")),
                _ => None,
            };
            return match p1_result {
                Some((winner, result)) => (winner.to_string(), result.to_string()),
                None => {
                    p2_result = match self.results.get(&p2).unwrap().as_str() {
                        "Crash" => Some((p1.as_ref(), "Player2Crash")),
                        "Timeout" => Some((p1.as_ref(), "Player2TimeOut")),
                        "Victory" => Some((p2.as_ref(), "Player2Win")),
                        "Defeat" => Some((p1.as_ref(), "Player1Win")),
                        "Tie" => Some(("Tie", "Tie")),
                        "InitializationError" => Some(("", "InitializationError")),
                        _ => None,
                    };
                    match p2_result {
                        Some((winner, result)) => (winner.to_string(), result.to_string()),
                        None => ("".to_string(), "Error".to_string()),
                    }
                }
            };
        };
        ("".to_string(), "Error".to_string())
    }
    #[allow(dead_code)]
    pub fn get_map(&self) -> String {
        self.map.clone()
    }
}

impl Into<GameResult> for ResultsData {
    fn into(self) -> GameResult {
        let (winner, result) = self.get_winner_result();
        let p1 = self.get_bot1();
        let p2 = self.get_bot2();
        GameResult {
            match_id: self.match_id,
            bot1: self.get_bot1(),
            bot2: self.get_bot2(),
            winner,
            map: self.map,
            result,
            game_time: self.game_time,
            game_time_formatted: "".to_string(),
            time_stamp: "".to_string(),
            bot1_avg_frame: self
                .average_frame_time
                .get(&p1)
                .unwrap_or(&Some(0.0f64))
                .unwrap_or(0.0f64),
            bot2_avg_frame: self
                .average_frame_time
                .get(&p2)
                .unwrap_or(&Some(0.0f64))
                .unwrap_or(0.0f64),
            replay_path: self.replay_path,
        }
    }
}
// This is a workaround due to the rust-arenaclient's json result not having a set structure
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ErrorResultsData {
    result: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct GameResult {
    match_id: i64,
    bot1: String,
    bot2: String,
    winner: String,
    map: String,
    result: String,
    game_time: u64,
    game_time_formatted: String,
    time_stamp: String,
    bot1_avg_frame: f64,
    bot2_avg_frame: f64,
    replay_path: String,
}

pub fn save_to_file<T: Serialize>(data: &T, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut f: File;
    if !Path::new(file_name).exists() {
        f = File::create(file_name)?;
    } else {
        f = OpenOptions::new().write(true).open(file_name)?;
    }
    // Clear file
    f.set_len(0)?;
    f.write_all((serde_json::to_string_pretty(&data)?).as_bytes())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Default)]
pub struct FileResultsData {
    #[serde(default, rename = "Results")]
    results: Vec<GameResult>,
}

impl FileResultsData {
    #[allow(dead_code)]
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
        let data: FileResultsData = serde_json::from_str(&contents)?;

        Ok(data)
    }
    #[allow(dead_code)]
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
    pub fn add_result(&mut self, result: GameResult) {
        self.results.push(result)
    }
    pub fn max_match_id(&self) -> i64 {
        self.results
            .iter()
            .map(|x| x.match_id)
            .max()
            .unwrap_or(0i64)
    }
}
