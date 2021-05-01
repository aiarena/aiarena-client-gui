use aiarena_client_gui_backend_lib::project_directory;
use serde::Deserialize;
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
pub fn open_file_dialog() -> String {
  match FileDialogBuilder::default().pick_folder() {
    None => "".to_string(),
    Some(path) => path.display().to_string(),
  }
}
#[tauri::command]
pub fn tauri_test() -> bool {
  true
}

#[tauri::command]
pub fn get_project_directory() -> String {
  match project_directory() {
    Ok(path) => path,
    Err(e) => e.to_string(),
  }
}
