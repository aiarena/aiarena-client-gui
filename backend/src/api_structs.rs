use paperclip::actix::Apiv2Schema;

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct Maps {
    #[serde(default, rename = "Maps")]
    pub(crate) maps: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct Bots {
    #[serde(default, rename = "Bots")]
    pub(crate) bots: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct AiarenaApiBots {
    count: u64,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<AiarenaApiBot>,
}

#[derive(Deserialize, Serialize, Debug, Default, Apiv2Schema)]
pub struct AiarenaApiBot {
    id: i64,
    user: i64,
    name: String,
    created: String,
    plays_race: String,
    bot_type: String,
    game_display_id: String,
    bot_zip_updated: String,
    bot_zip_publicly_downloadable: bool,
    bot_zip: String,
    bot_zip_md5hash: String,
    bot_data_publicly_downloadable: bool,
    bot_data: String,
    bot_data_md5hash: String,
}
