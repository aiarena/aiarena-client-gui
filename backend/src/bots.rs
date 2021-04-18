use crate::routes::CLIENT_PORT;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub fn start_bot(
    bot_name: String,
    bots_directory: String,
    opp_id: String,
) -> Result<Child, Box<dyn std::error::Error>> {
    let bot_path = Path::new(&bots_directory).join(&bot_name);
    if !bot_path.exists() {
        return Err(format!("{} does not exist", bot_path.to_str().unwrap()).into());
    };
    if !bot_path.join("data").exists() {
        std::fs::create_dir(bot_path.join("data"))?;
    };
    let ladder_bots_json = bot_path.join("ladderbots.json");
    let bot = read_ladderbots_json(ladder_bots_json)?;
    let new_path: PathBuf;
    let port = CLIENT_PORT.to_string();
    let mut commands = vec![
        bot.file_name.as_str(),
        "--GamePort",
        &port,
        "--StartPort",
        &port,
        "--LadderServer",
        "127.0.0.1",
        "--OpponentID",
        opp_id.as_str(),
    ];
    let lower_type = bot.bot_type.to_ascii_lowercase();
    let start_command = match lower_type.as_str() {
        "dotnetcore" => "dotnet",
        "binarycpp" => {
            new_path = Path::new(&bots_directory).join(&bot.file_name);
            commands.remove(0);
            new_path.to_str().unwrap()
        }
        "java" => {
            commands.insert(0, "-jar");
            "java"
        }
        "nodejs" => "node",
        name => name,
    };

    let stderr_log = File::create(bot_path.join("data").join("stderr.log")).unwrap();
    let stdout_log = stderr_log.try_clone()?;
    let mut command = Command::new(start_command);
    let c = command
        .args(commands)
        .current_dir(&bot_path)
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log));

    Ok(c.spawn()?)
}

fn read_ladderbots_json(path: PathBuf) -> Result<BotJson, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut ladder_bots_json: LadderBotsJson = serde_json::from_reader(reader)?;
    let mut bot: BotJson = Default::default();
    if ladder_bots_json.bots.is_empty() {
        Err("ladderbots.json error".into())
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
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BotJson {
    #[serde(default, skip_deserializing)]
    name: String,
    #[serde(rename = "Race")]
    race: String,
    #[serde(rename = "Type")]
    bot_type: String,
    #[serde(rename = "RootPath")]
    root_path: String,
    #[serde(rename = "FileName")]
    file_name: String,
    #[serde(rename = "Debug")]
    debug: bool,
}

#[cfg(test)]
mod tests {
    use crate::bots::{read_ladderbots_json, start_bot};
    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn test_read_ladderbots() {
        let ladder_bots_json = Path::new(r#"../backend/test_data/ladderbots.json"#).to_path_buf();
        let bot = read_ladderbots_json(ladder_bots_json).unwrap();
        assert!(!bot.name.is_empty())
    }
}
