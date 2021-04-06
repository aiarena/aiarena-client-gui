#![allow(dead_code)]

use crate::paths;
use crate::settings_data::SettingsFormData;
use regex::Regex;
use std::env::var_os;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn default_base() -> PathBuf {
    // TODO: Detect Wine and use "~/.wine/drive_c/Program Files (x86)/StarCraft II"

    if let Some(base_dir) = var_os("SC2_PROXY_BASE") {
        Path::new(&base_dir).to_path_buf()
    } else if cfg!(windows) {
        Path::new("C:/Program Files (x86)/StarCraft II").to_path_buf()
    } else if cfg!(target_os = "macos") {
        Path::new("/Applications/StarCraft II").to_path_buf()
    } else if cfg!(linux) {
        Path::new(&shellexpand::tilde("~/StarCraftII").into_owned()).to_path_buf()
    } else {
        panic!("Unknown system, use SC2_PROXY_BASE env var");
    }
}

/// SC2 binary path inside the correct version folder
fn bin_path() -> PathBuf {
    if let Some(base_dir) = var_os("SC2_PROXY_BIN") {
        Path::new(&base_dir).to_path_buf()
    } else if cfg!(windows) {
        Path::new("SC2_x64.exe").to_path_buf()
    } else if cfg!(target_os = "macos") {
        Path::new("SC2.app/Contents/MacOS/SC2").to_path_buf()
    } else if cfg!(linux) {
        Path::new(&shellexpand::tilde("SC2_x64").into_owned()).to_path_buf()
    } else {
        panic!("Unknown system, use SC2_PROXY_BIN env var");
    }
}

/// The working directory to use inside the base dir
fn cwd() -> Option<PathBuf> {
    if let Some(base_dir) = var_os("SC2_PROXY_CWD") {
        Some(Path::new(&base_dir).to_path_buf())
    } else if cfg!(windows) {
        Some(Path::new("Support64").to_path_buf())
    } else {
        None
    }
}

fn latest_executable_path(versions_dir: PathBuf) -> PathBuf {
    let (max_version, path) = fs::read_dir(versions_dir)
        .unwrap()
        .filter_map(|entry| -> Option<(u64, PathBuf)> {
            let path = entry.unwrap().path();
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .expect("Invalid unicode in folder name");

            if path.metadata().unwrap().is_dir() && name.starts_with("Base") {
                let version: &str = name.split_at(4).1;
                version.parse::<u64>().ok().map(|v| (v, path.to_path_buf()))
            } else {
                None
            }
        })
        .max_by_key(|(v, _)| *v)
        .expect("No downloaded SC2 binaries found");

    if max_version < 55958 {
        panic!("Your SC2 binary is too old. Upgrade to 3.16.1 or newer.");
    }

    path.join(bin_path())
}

fn execute_info_path() -> Option<PathBuf> {
    let env_skip_os_str = var_os("SC2_PROXY_SKIP_EXECUTE_INFO").unwrap_or_default();
    let env_skip_str = env_skip_os_str
        .to_str()
        .expect("SC2_PROXY_SKIP_EXECUTE_INFO was invalid unicode");

    if env_skip_str.is_empty() || env_skip_str == "0" {
        None
    } else if cfg!(windows) {
        Some(
            Path::new(
                &shellexpand::tilde("~\\Documents\\StarCraft II\\ExecuteInfo.txt").into_owned(),
            )
            .to_path_buf(),
        )
    } else if cfg!(target_os = "macos") {
        Some(
            Path::new("/Library/Application Support/Blizzard/StarCraft II/ExecuteInfo.txt")
                .to_path_buf(),
        )
    } else {
        None
    }
}

// Reads ExecuteInfo.txt, if available
fn read_execute_info(path: PathBuf) -> Option<PathBuf> {
    let mut f = fs::File::open(path).ok()?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read ExecuteInfo.txt");

    let re = Regex::new(r" = (.*)Versions").unwrap();
    let base = Path::new(re.captures(&contents)?.get(1).unwrap().as_str()).to_path_buf();

    if base.exists() {
        Some(base)
    } else {
        None
    }
}

pub fn normalize_map_name(map_name: &Path) -> String {
    map_name
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace(".SC2Map", "")
}
/// Basedir, tries to use ExecuteInfo.txt first
pub fn base_dir() -> PathBuf {
    if let Some(base_dir) = var_os("SC2_PROXY_BASE") {
        Path::new(&base_dir).to_path_buf()
    } else if let Some(ei_path) = execute_info_path() {
        read_execute_info(ei_path).unwrap_or_else(default_base)
    } else {
        default_base()
    }
}

/// PathBuf to SC2 binary executable
pub fn executable() -> PathBuf {
    latest_executable_path(base_dir().join(Path::new("Versions")))
}

/// Directory to switch to before starting SC2
pub fn cwd_dir() -> PathBuf {
    let base = base_dir();
    if let Some(c) = cwd() {
        base.join(c)
    } else {
        base
    }
}

/// Directory containing replays
pub fn replay_dir() -> PathBuf {
    base_dir().join(Path::new("Replays").to_path_buf())
}

/// Directory containing map directories
pub fn map_dir() -> PathBuf {
    // TODO: lowercase variant?
    base_dir().join(Path::new("Maps").to_path_buf())
}

pub fn find_available_maps() -> Vec<String> {
    let mapdir = paths::map_dir();
    let mut maps = vec![];
    for outer in fs::read_dir(&mapdir).expect("Could not iterate map directory") {
        let outer_path = outer.unwrap().path();
        if !outer_path.is_dir() {
            if outer_path.extension().unwrap() == "SC2Map" {
                let relative = outer_path.strip_prefix(&mapdir).unwrap();
                maps.push(normalize_map_name(relative));
                continue;
            } else {
                continue;
            }
        }

        for inner in fs::read_dir(outer_path).expect("Could not iterate map subdirectory") {
            let path = inner.unwrap().path();
            if !path.is_dir() {
                if path.extension().unwrap() == ".SC2Map" {
                    let relative = path.strip_prefix(&mapdir).unwrap();
                    maps.push(normalize_map_name(relative));
                } else {
                    continue;
                }
            }
        }
    }
    maps
}

pub fn find_available_bots() -> Vec<String> {
    let mut bots = vec![];
    if let Ok(settings_data) = SettingsFormData::load_from_file() {
        let bot_dir = Path::new(&settings_data.bot_directory_location).to_path_buf();
        if bot_dir.exists() && bot_dir.is_dir() {
            for outer in fs::read_dir(&bot_dir).expect("Could not iterate bot directory") {
                let outer_path = outer.unwrap().path();
                if outer_path.is_dir() {
                    for inner in
                        fs::read_dir(&outer_path).expect("Could not iterate map subdirectory")
                    {
                        let path = inner.unwrap().path();
                        if !path.is_dir() {
                            match path.file_name() {
                                Some(file_name) => {
                                    if file_name == "ladderbots.json" {
                                        let relative = outer_path.strip_prefix(&bot_dir).unwrap();

                                        bots.push(relative.to_str().unwrap().to_string());
                                        continue;
                                    }
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    bots
}
