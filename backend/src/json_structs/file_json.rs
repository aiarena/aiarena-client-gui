use crate::errors::MyError;
use anyhow::Result;
use fs2::FileExt;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

#[allow(clippy::upper_case_acronyms)]
pub trait JSONFile {
    fn file_path() -> Result<PathBuf, MyError>;
    fn save_to_file(&self) -> Result<(), MyError>
    where
        Self: std::marker::Sized + Serialize,
    {
        let mut f: File;
        let file = Self::file_path()?;
        if !file.exists() {
            f = File::create(file)?;
        } else {
            f = OpenOptions::new().write(true).open(file)?;
        }
        f.lock_exclusive()?;

        // Clear file
        f.set_len(0)?;
        let serialized = serde_json::to_string_pretty(&self)?;
        f.write_all(serialized.as_bytes())?;
        f.unlock()?;
        Ok(())
    }
    fn load_from_file() -> Result<Self, MyError>
    where
        Self: std::marker::Sized + DeserializeOwned + Default + Serialize,
    {
        let mut f: File;
        let file = Self::file_path()?;
        if !file.exists() {
            f = OpenOptions::new().write(true).create(true).open(&file)?;
            let s = Self::default();
            f.write_all((serde_json::to_string_pretty(&s)?).as_bytes())?;
        } else {
            f = File::open(&file)?;
        }
        let mut contents = String::new();
        f.lock_exclusive()?;
        f.read_to_string(&mut contents)?;
        // Deserialize and populate Rust data structure.
        let data: Self = serde_json::from_str(&contents)?;
        f.unlock()?;

        Ok(data)
    }
}
