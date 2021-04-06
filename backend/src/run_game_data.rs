use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct RunGameData {
    #[serde(rename = "Bot1")]
    pub bot1: Vec<String>,
    #[serde(rename = "Bot2")]
    pub bot2: Vec<String>,
    #[serde(rename = "Map")]
    pub map: Vec<String>,
    #[serde(rename = "Iterations")]
    pub iterations: i32,
    #[serde(rename = "Visualize")]
    pub visualize: bool,
    #[serde(rename = "Realtime")]
    pub realtime: bool,
}
