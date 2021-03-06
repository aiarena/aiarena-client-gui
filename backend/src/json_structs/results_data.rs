use rust_ac::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
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
    pub fn new(
        results: HashMap<String, String>,
        game_time: u64,
        game_time_seconds: f64,
        average_frame_time: HashMap<String, Option<f64>>,
        status: String,
        bots: HashMap<u8, String>,
        map: String,
        replay_path: String,
        match_id: i64,
    ) -> Self {
        Self {
            results,
            game_time,
            game_time_seconds,
            average_frame_time,
            status,
            bots,
            map,
            replay_path,
            match_id,
        }
    }
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
    pub fn init_error(config: &Config) -> Self {
        let mut results = HashMap::new();
        let mut average_frame_time = HashMap::new();
        let mut bots = HashMap::new();
        results.insert(
            config.player1().to_string(),
            "InitializationError".to_string(),
        );
        results.insert(
            config.player2().to_string(),
            "InitializationError".to_string(),
        );
        average_frame_time.insert(config.player1().to_string(), None);
        average_frame_time.insert(config.player2().to_string(), None);
        bots.insert(1, config.player1().to_string());
        bots.insert(2, config.player2().to_string());
        Self {
            results,
            game_time: 0,
            game_time_seconds: 0.0,
            average_frame_time,
            status: "Completed".to_string(),
            bots,
            map: config.map().clone(),
            replay_path: "".to_string(),
            match_id: config.match_id,
        }
    }
}

impl From<ResultsData> for GameResult {
    fn from(results_data: ResultsData) -> Self {
        let (winner, result) = results_data.get_winner_result();
        let p1 = results_data.get_bot1();
        let p2 = results_data.get_bot2();
        Self {
            match_id: results_data.match_id,
            bot1: results_data.get_bot1(),
            bot2: results_data.get_bot2(),
            winner,
            map: results_data.map,
            result,
            game_time: results_data.game_time,
            game_time_formatted: "".to_string(),
            time_stamp: "".to_string(),
            bot1_avg_frame: results_data
                .average_frame_time
                .get(&p1)
                .unwrap_or(&Some(0.0f64))
                .unwrap_or(0.0f64),
            bot2_avg_frame: results_data
                .average_frame_time
                .get(&p2)
                .unwrap_or(&Some(0.0f64))
                .unwrap_or(0.0f64),
            replay_path: results_data.replay_path,
        }
    }
}
// This is a workaround due to the rust-arenaclient's json result not having a set structure
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ErrorResultsData {
    result: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub(crate) match_id: i64,
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

#[cfg(test)]
mod tests {
    use crate::json_structs::results_data::{GameResult, ResultsData};
    use std::collections::HashMap;

    fn results_data_setup() -> ResultsData {
        let mut results = HashMap::new();
        let mut average_frame_time = HashMap::new();
        let mut bots = HashMap::new();
        results.insert("Bot1".to_string(), "Victory".to_string());
        results.insert("Bot2".to_string(), "Defeat".to_string());
        average_frame_time.insert("Bot1".to_string(), Some(0.1));
        average_frame_time.insert("Bot2".to_string(), Some(0.2));
        bots.insert(1, "Bot1".to_string());
        bots.insert(2, "Bot2".to_string());

        ResultsData::new(
            results,
            60486,
            2700.267857142857,
            average_frame_time,
            "Complete".to_string(),
            bots,
            "AutomatonLE".to_string(),
            "/empty/path".to_string(),
            1,
        )
    }
    #[test]
    fn test_get_bot1() {
        let results_data = results_data_setup();
        assert_eq!(results_data.get_bot1(), "Bot1".to_string())
    }
    #[test]
    fn test_get_bot2() {
        let results_data = results_data_setup();
        assert_eq!(results_data.get_bot2(), "Bot2".to_string())
    }
    #[test]
    fn test_get_map() {
        let results_data = results_data_setup();
        assert_eq!(results_data.get_map(), "AutomatonLE".to_string())
    }
    #[test]
    fn test_get_winner_result() {
        let results_data = results_data_setup();
        let winner_result = results_data.get_winner_result();
        assert_eq!(winner_result.0, "Bot1");
        assert_eq!(winner_result.1, "Player1Win");
    }
    #[test]
    fn test_game_result_conversion() {
        let results_data = results_data_setup();
        let game_result: GameResult = results_data.clone().into();
        assert_eq!(game_result.match_id, results_data.match_id);
        assert_eq!(game_result.map, results_data.get_map());
        assert_eq!(game_result.result, results_data.get_winner_result().1);
        assert_eq!(game_result.replay_path, results_data.replay_path);
        assert_eq!(game_result.game_time, results_data.game_time);
        assert_eq!(game_result.winner, results_data.get_winner_result().0);
        assert_eq!(game_result.bot1, results_data.get_bot1());
        assert_eq!(game_result.bot2, results_data.get_bot2());
    }
}
