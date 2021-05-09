use crate::{code_loc, server::api_structs::AiarenaApiBot};
use anyhow::{Context, Result};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

pub fn read_ladderbots_json(path: PathBuf) -> Result<BotJson> {
    let file = File::open(path).context(code_loc!())?;
    let reader = BufReader::new(file);
    let mut ladder_bots_json: LadderBotsJson =
        serde_json::from_reader(reader).context(code_loc!())?;
    let mut bot: BotJson = Default::default();
    if ladder_bots_json.bots.is_empty() {
        bail!("ladderbots.json error")
    } else {
        for (x, y) in ladder_bots_json.bots.iter_mut() {
            bot = y.clone();
            bot.name = x.clone();
        }
        Ok(bot)
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LadderBotsJson {
    #[serde(rename = "Bots")]
    bots: HashMap<String, BotJson>,
}
impl LadderBotsJson {
    #[allow(dead_code)]
    pub fn from(ai_arena_bot: &AiarenaApiBot) {
        let mut bots = HashMap::new();
        bots.insert(
            ai_arena_bot.name.clone(),
            BotJson {
                name: ai_arena_bot.name.clone(),
                race: ai_arena_bot.plays_race.clone(),
                bot_type: ai_arena_bot.bot_type.clone(),
                root_path: "".to_string(),
                file_name: "".to_string(),
                debug: false,
            },
        );
    }
    #[allow(dead_code)]
    pub fn save_to_file(_path: PathBuf) -> Result<()> {
        Ok(())
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BotJson {
    #[serde(default, skip_deserializing)]
    name: String,
    #[serde(rename = "Race")]
    race: String,
    #[serde(rename = "Type")]
    pub(crate) bot_type: String,
    #[serde(rename = "RootPath")]
    root_path: String,
    #[serde(rename = "FileName")]
    pub(crate) file_name: String,
    #[serde(rename = "Debug")]
    debug: bool,
}

#[cfg(test)]
mod tests {
    use crate::bot::ladder_bots::read_ladderbots_json;
    use std::path::Path;

    #[test]
    fn test_read_ladderbots() {
        let ladder_bots_json = Path::new(r#"../backend/test_data/ladderbots.json"#).to_path_buf();
        let bot = read_ladderbots_json(ladder_bots_json).unwrap();
        assert!(!bot.name.is_empty())
    }
}
