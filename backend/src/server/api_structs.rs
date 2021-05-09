#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Maps {
    #[serde(default, rename = "Maps")]
    pub(crate) maps: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Bots {
    #[serde(default, rename = "Bots")]
    pub(crate) bots: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AiarenaApiBots {
    count: u64,
    next: Option<String>,
    previous: Option<String>,
    pub(crate) results: Vec<AiarenaApiBot>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AiarenaApiBot {
    pub id: i64,
    pub user: i64,
    pub name: String,
    pub created: String,
    pub plays_race: String,
    #[serde(rename = "type")]
    pub bot_type: String,
    pub game_display_id: String,
    pub bot_zip_updated: Option<String>,
    pub bot_zip_publicly_downloadable: bool,
    pub bot_zip: Option<String>,
    pub bot_zip_md5hash: Option<String>,
    pub bot_data_publicly_downloadable: bool,
    pub bot_data: Option<String>,
    pub bot_data_md5hash: Option<String>,
}
