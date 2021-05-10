use aiarena_client_gui_backend_lib::project_directory;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::process::{exit, Command, Stdio};
use tauri::api::app::current_binary;
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::shell::open;

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
  if cfg!(target_os = "linux") {
    false
  } else {
    true
  }
}

#[tauri::command]
pub fn get_project_directory() -> String {
  match project_directory() {
    Ok(path) => path,
    Err(e) => e.to_string(),
  }
}

#[tauri::command]
pub fn get_debug_logs_directory() -> String {
  match project_directory() {
    Ok(path) => {
      let debug_dir = Path::new(&path).join("debug_logs");
      std::fs::create_dir_all(debug_dir.clone()).unwrap();
      debug_dir.display().to_string()
    }
    Err(e) => e.to_string(),
  }
}
#[tauri::command]
pub fn open_directory(path: String) {
  open(path, None).unwrap();
}

#[tauri::command]
pub fn restart_app_with_logs(env_var: String) {
  let binary_path = current_binary();
  if let Some(path) = binary_path {
    let debug_dir = Path::new(&project_directory().unwrap()).join("debug_logs");
    std::fs::create_dir_all(debug_dir.clone()).unwrap();
    let filename = format!(
      "aiarena-client-gui{}.log",
      chrono::Utc::now()
        .to_string()
        .replace("-", "_")
        .replace(" ", "_")
        .replace(".", "")
        .replace(":", "")
    );
    println!("{}", &filename);

    let stderr_log = File::create(debug_dir.join(&filename)).unwrap();
    let stdout_log = stderr_log.try_clone().unwrap();
    Command::new(path)
      .args(&["--log".to_string(), env_var])
      .stdout(Stdio::from(stdout_log))
      .stderr(Stdio::from(stderr_log))
      .spawn()
      .expect("application failed to start");
  }
  exit(0)
}
