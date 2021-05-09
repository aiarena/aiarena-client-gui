use anyhow::Result;
use fs2::FileExt;
use serde::Serialize;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

pub mod md5_hashes_file;
pub mod results_data_file;
pub mod settings_data_file;

#[allow(dead_code)]
pub fn save_to_file<T: Serialize>(data: &T, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut f: File;
    if !Path::new(file_name).exists() {
        f = File::create(file_name)?;
    } else {
        f = OpenOptions::new().write(true).open(file_name)?;
    }
    f.lock_exclusive()?;

    // Clear file
    f.set_len(0)?;
    f.write_all((serde_json::to_string_pretty(&data)?).as_bytes())?;
    f.unlock()?;
    Ok(())
}
