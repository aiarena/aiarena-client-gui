use crate::{bot::ladder_bots::read_ladderbots_json, server::routes::CLIENT_PORT};
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

pub mod aiarena_bots;
pub mod ladder_bots;

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
        "--OpponentId",
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
    let data_dir = bot_path.join("data");
    std::fs::create_dir_all(data_dir.clone()).unwrap();
    let stderr_log = File::create(data_dir.join("stderr.log")).unwrap();
    let stdout_log = stderr_log.try_clone()?;
    let mut command = Command::new(start_command);
    let c = command
        .args(commands)
        .current_dir(&bot_path)
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log));

    Ok(c.spawn()?)
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BotConnection {
    #[serde(rename = "Bot")]
    bot: String,
}
