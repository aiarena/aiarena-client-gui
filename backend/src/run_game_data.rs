use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct RunGameData {
    #[serde(default, rename = "Bot1")]
    pub bot1: Vec<String>,
    #[serde(default, rename = "Bot2")]
    pub bot2: Vec<String>,
    #[serde(default, rename = "Map")]
    pub map: Vec<String>,
    #[serde(default, rename = "Iterations")]
    pub iterations: u64,
    #[serde(default, rename = "Visualize")]
    pub visualize: bool,
    #[serde(default, rename = "Realtime")]
    pub realtime: bool,
}
