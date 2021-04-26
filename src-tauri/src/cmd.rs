use serde::Deserialize;
use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  MyCustomCommand { argument: String },
}

#[tauri::command]
pub fn my_custom_command() -> String {
  match FileDialogBuilder::default().pick_folder() {
    None => "".to_string(),
    Some(path) => path.display().to_string(),
  }
}
#[tauri::command]
pub fn tauri_test() -> bool {
  true
}
