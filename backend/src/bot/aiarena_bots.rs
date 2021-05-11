use crate::{
    code_loc,
    errors::MyError,
    files::{md5_hashes_file::MD5HashesFile, settings_data_file::SettingsFormData},
    helpers::get_non_empty_string,
    json_structs::file_json::JSONFile,
    server::{
        api_structs::{AiarenaApiBot, AiarenaApiBots},
        routes::AIARENA_URL,
    },
};
use actix_web::{client::Client, web::Buf};
use anyhow::{Context, Result};
use async_rwlock::RwLock;
use log::{debug, error};
use std::{
    fs::OpenOptions,
    io::{ErrorKind, Write},
    path::Path,
    time::Duration,
};
use zip::read::ZipArchive;

#[allow(dead_code)]
pub enum BotNumber {
    One,
    Two,
}
#[allow(dead_code)]
pub type OptionalReceiver = Option<crossbeam::channel::Receiver<BotDownloadStatus>>;
#[allow(dead_code)]
pub struct ToDownload {
    bot_statuses: (BotDownloadStatus, BotDownloadStatus),
    bot_channels: (OptionalReceiver, OptionalReceiver),
}
impl ToDownload {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            bot_statuses: (
                BotDownloadStatus::NotApplicable,
                BotDownloadStatus::NotApplicable,
            ),
            bot_channels: (None, None),
        }
    }
    #[allow(dead_code)]
    pub fn add_channel(&mut self, bot_number: BotNumber, channel: OptionalReceiver) {
        match bot_number {
            BotNumber::One => self.bot_channels.0 = channel,
            BotNumber::Two => self.bot_channels.1 = channel,
        }
    }
    #[allow(dead_code)]
    pub fn update_status(&mut self, bot_number: BotNumber, status: BotDownloadStatus) {
        match bot_number {
            BotNumber::One => self.bot_statuses.0 = status,
            BotNumber::Two => self.bot_statuses.1 = status,
        }
    }
    #[allow(dead_code)]
    pub fn update_statuses(&mut self) {
        match &self.bot_channels.0 {
            None => {}
            Some(channel) => {
                if let Ok(msg) = channel.try_recv() {
                    self.bot_statuses.0 = msg;
                }
            }
        }
        match &self.bot_channels.1 {
            None => {}
            Some(channel) => {
                if let Ok(msg) = channel.try_recv() {
                    self.bot_statuses.1 = msg;
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn wait_until_done(&mut self) {
        while !&self.bot_statuses.0.is_done() && !&self.bot_statuses.1.is_done() {
            std::thread::sleep(Duration::from_secs(2));
            self.update_statuses();
        }
    }
}

pub struct BotDownloadClient {
    client: Client,
    md5_hashes_file: RwLock<MD5HashesFile>,
}
impl BotDownloadClient {
    pub fn new() -> Self {
        let connector = actix_web::client::Connector::new()
            .timeout(Duration::from_secs(60))
            .finish();
        let client = Client::builder()
            .connector(connector)
            .timeout(Duration::from_secs(60))
            .finish();
        Self {
            client,
            md5_hashes_file: RwLock::new(MD5HashesFile::load_from_file().unwrap_or_default()),
        }
    }
    pub async fn get_and_download_all<'a>(
        &self,
        bots: &'a [String],
    ) -> Result<Vec<(&'a String, Result<()>)>> {
        let normalized_bots: Vec<String> = bots
            .iter()
            .map(|bot_name| bot_name.replace(" (AI-Arena)", ""))
            .collect();
        let settings_data = SettingsFormData::load_from_file().context(code_loc!())?;
        let mut response = self
            .client
            .get(format!(
                "{}{}",
                AIARENA_URL,
                r#"/api/bots/?&format=json&bot_zip_publicly_downloadable=true&ordering=name"#
            ))
            .timeout(Duration::from_secs(60))
            .header("User-Agent", "Actix-web")
            .header(
                "Authorization",
                format!("Token  {}", settings_data.api_token),
            )
            .send() // <- Send https request
            .await
            .map_err(MyError::from)?;

        let body = response
            .body()
            .limit(usize::MAX)
            .await
            .context(code_loc!())?;

        let s = String::from_utf8(body.to_vec()).context(code_loc!())?;

        let aiarena_api_bots: AiarenaApiBots =
            serde_json::from_str(&s).context(code_loc!()).unwrap();

        let mut futures = Vec::new();
        for bot in aiarena_api_bots
            .results
            .iter()
            .filter(|&ai_bot| normalized_bots.contains(&ai_bot.name))
        {
            futures.push(self.download_zip(bot));
        }
        let r = futures::future::join_all(futures).await;
        Ok(bots.iter().zip(r).collect())
    }
    pub async fn get_and_download_bot_zip(&self, bot_name: String) -> Result<()> {
        let bot_name = bot_name.replace(" (AI-Arena)", "");
        let bot = self.get_bot_details(bot_name).await?;
        self.download_zip(&bot).await
    }
    pub async fn get_bot_details(&self, bot_name: String) -> Result<AiarenaApiBot> {
        let settings_data = SettingsFormData::load_from_file()?;
        let mut response = self
            .client
            .get(format!(
                "{}{}{}",
                AIARENA_URL, r#"/api/bots/?format=json&name="#, bot_name
            ))
            .timeout(Duration::from_secs(60))
            .header("User-Agent", "Actix-web")
            .header(
                "Authorization",
                format!("Token  {}", settings_data.api_token),
            )
            .send() // <- Send https request
            .await
            .map_err(MyError::from)?;
        let body = response
            .body()
            .limit(usize::MAX)
            .await
            .context(code_loc!())?;

        let s = String::from_utf8(body.to_vec()).context(code_loc!())?;

        let aiarena_api_bots: AiarenaApiBots =
            serde_json::from_str(&s).context(code_loc!()).unwrap();

        aiarena_api_bots
            .results
            .get(0)
            .ok_or_else(|| anyhow!("Could not connect to AiArena API".to_string()))
            .map(|x| x.clone())
            .context(code_loc!())
    }
    pub async fn download_zip(&self, bot: &AiarenaApiBot) -> Result<()> {
        let settings_data = SettingsFormData::load_from_file().context(code_loc!())?;
        let config_bot_dir =
            get_non_empty_string(settings_data.bot_directory_location).context(code_loc!())?;
        let bot_directory = Path::new(&config_bot_dir);

        let mut redownload: bool = match self.md5_hashes_file.read().await.get_hash(&bot.name) {
            None => true,
            Some(bot_hash) => bot.bot_data_md5hash.as_ref() != bot_hash.get_zip_hash(),
        };
        if !redownload {
            redownload = !bot_directory
                .read_dir()
                .context(code_loc!())
                .unwrap()
                .any(|x| {
                    if let Ok(entry) = x {
                        if entry.path().is_dir() {
                            entry.path().display().to_string().contains(&bot.name)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
        }
        if redownload {
            debug!("Redownload bot");
            if bot.bot_zip.is_some() {
                let mut response = self
                    .client
                    .get(bot.bot_zip.as_ref().unwrap())
                    .timeout(Duration::from_secs(60))
                    .header("User-Agent", "Actix-web")
                    .header(
                        "Authorization",
                        format!("Token  {}", settings_data.api_token),
                    )
                    .timeout(Duration::from_secs(60))
                    .send()
                    .await
                    .map_err(MyError::from)
                    .context(code_loc!())?; // <- Send https request
                let body = response
                    .body()
                    .limit(usize::MAX)
                    .await
                    .context(code_loc!())?;

                let zip_name = format!("{} (AI-Arena).zip", &bot.name);
                let zip_directory = bot_directory.join("AI-Arena Zips");
                let full_zip_name = zip_directory.join(&zip_name);
                match std::fs::create_dir(zip_directory.clone()) {
                    Ok(_) => {}
                    Err(err) => {
                        if err.kind() != ErrorKind::AlreadyExists {
                            error!(
                                "{:?} - {} - {:?}",
                                &zip_directory,
                                err.to_string(),
                                code_loc!()
                            )
                        }
                    }
                };
                let mut zip_file = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(full_zip_name)
                    .context(code_loc!())?;
                zip_file.write_all(body.bytes()).context(code_loc!())?;
                let mut archive = ZipArchive::new(zip_file).context(code_loc!())?;
                let zip_target = bot_directory.join(format!("{} (AI-Arena)", &bot.name));
                archive.extract(zip_target).context(code_loc!())?;
            }
        }
        debug!("Update hashes");
        self.md5_hashes_file
            .write()
            .await
            .update_zip_hash(&bot.name, bot.bot_zip_md5hash.as_ref().unwrap());
        self.md5_hashes_file
            .read()
            .await
            .save_to_file()
            .context(code_loc!())?;
        Ok(())
    }
}
#[allow(dead_code)]
pub enum BotDownloadStatus {
    NotApplicable,
    Pending,
    DetailRequest,
    ZipDownload,
    ZipExtract,
    Complete,
    Error,
}
impl BotDownloadStatus {
    #[allow(dead_code)]
    pub fn is_done(&self) -> bool {
        matches!(
            self,
            BotDownloadStatus::NotApplicable
                | BotDownloadStatus::Complete
                | BotDownloadStatus::Error
        )
    }
}
#[allow(dead_code)]
pub async fn download_bot(bot_name: String) {
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        let bot_name = bot_name.replace(" (AI-Arena)", "");
        let client = Client::default();

        let mut response = client
            .get(format!(
                "{}{}{}",
                AIARENA_URL, r#"/api/bots/?format=json&name="#, bot_name
            )) // <- Create request builder
            .header("User-Agent", "Actix-web")
            .header(
                "Authorization",
                format!("Token  {}", settings_data.api_token),
            )
            .send() // <- Send https request
            .await
            .unwrap();

        let body = response
            .body()
            .limit(usize::MAX)
            .await
            .context(code_loc!())
            .unwrap();

        let s = String::from_utf8(body.to_vec())
            .context(code_loc!())
            .unwrap();

        let aiarena_api_bots: AiarenaApiBots =
            serde_json::from_str(&s).context(code_loc!()).unwrap();

        let bot = aiarena_api_bots
            .results
            .get(0)
            .ok_or_else(|| anyhow!("Could not connect to AiArena API".to_string()))
            .context(code_loc!())
            .unwrap();
        let bot_directory = Path::new(&settings_data.bot_directory_location);
        let mut md5_hashes_files = MD5HashesFile::load_from_file().unwrap();
        let mut redownload: bool = match md5_hashes_files.get_hash(&bot.name) {
            None => true,
            Some(bot_hash) => bot.bot_data_md5hash.as_ref() != bot_hash.get_zip_hash(),
        };
        if !redownload {
            redownload = !bot_directory
                .read_dir()
                .context(code_loc!())
                .unwrap()
                .any(|x| {
                    if let Ok(entry) = x {
                        if entry.path().is_dir() {
                            entry.path().display().to_string().contains(&bot.name)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
        }
        if redownload {
            debug!("Redownload bot");
            if bot.bot_zip.is_some() {
                let mut response = client
                    .get(bot.bot_zip.as_ref().unwrap())
                    .header("User-Agent", "Actix-web")
                    .header(
                        "Authorization",
                        format!("Token  {}", settings_data.api_token),
                    )
                    .timeout(Duration::from_secs(10))
                    .send()
                    .await
                    .unwrap(); // <- Send https request
                let body = response.body().limit(usize::MAX).await.unwrap();
                let zip_name = format!("{} (AI-Arena).zip", &bot.name);
                let zip_directory = bot_directory.join("AI-Arena Zips");
                let full_zip_name = zip_directory.join(&zip_name);
                match std::fs::create_dir(zip_directory) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("{}", err.to_string())
                    }
                };
                let mut zip_file = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(full_zip_name)
                    .context(code_loc!())
                    .unwrap();
                zip_file
                    .write_all(body.bytes())
                    .context(code_loc!())
                    .unwrap();
                let mut archive = ZipArchive::new(zip_file).context(code_loc!()).unwrap();
                let zip_target = bot_directory.join(format!("{} (AI-Arena)", &bot.name));
                archive.extract(zip_target).context(code_loc!()).unwrap();
            }
        }
        debug!("Update hashes");
        md5_hashes_files.update_zip_hash(&bot.name, bot.bot_zip_md5hash.as_ref().unwrap());
        md5_hashes_files.save_to_file().unwrap();
    }
}
