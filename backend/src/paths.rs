#![allow(dead_code)]

use crate::{
    files::settings_data_file::SettingsFormData, json_structs::file_json::JSONFile, paths,
};
use rust_ac::paths::*;
use std::{fs, path::Path};

pub fn normalize_map_name(map_name: &Path) -> String {
    map_name
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace(".SC2Map", "")
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
                if path.extension().unwrap() == "SC2Map" {
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
