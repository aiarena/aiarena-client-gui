use crate::{
    errors::MyError,
    json_structs::{file_json::JSONFile, results_data::GameResult},
};
use directories::ProjectDirs;
use std::path::PathBuf;

pub static RESULTS_FILE: &str = "results.json";
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileResultsData {
    #[serde(default, rename = "Results")]
    results: Vec<GameResult>,
}

impl FileResultsData {
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
impl JSONFile for FileResultsData {
    fn file_path() -> Result<PathBuf, MyError> {
        let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
            .ok_or_else(|| MyError::new("Could not create Project Directory"))?;
        if !project_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(project_dirs.data_local_dir())?;
        }
        Ok(project_dirs.data_local_dir().join(&RESULTS_FILE))
    }
}
