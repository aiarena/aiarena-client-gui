use crate::{MainWindow, SplashscreenWindow};

use aiarena_client_gui_backend_lib::project_directory;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::process::{exit, Command, Stdio};
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::process::current_binary;
use tauri::api::shell::open;
use tauri::State;

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
  let (tx, rx) = std::sync::mpsc::channel();

  FileDialogBuilder::new().pick_folder(move |paths| tx.send(paths).unwrap());

  rx.recv()
    .unwrap()
    .unwrap_or_default()
    .into_os_string()
    .into_string()
    .unwrap_or_default()
}
#[tauri::command]
pub fn tauri_test() -> bool {
  !cfg!(target_os = "linux")
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
pub fn settings_okay() -> bool {
  aiarena_client_gui_backend_lib::files::settings_data_file::settings_okay()
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
        .replace('-', "_")
        .replace(' ', "_")
        .replace('.', "")
        .replace(':', "")
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

#[tauri::command]
pub fn close_splashscreen(
  splashscreen: State<'_, SplashscreenWindow>,
  main: State<'_, MainWindow>,
) {
  // Close splashscreen
  splashscreen.0.lock().unwrap().close().unwrap();
  if !std::env::args().any(|x| x == *"--headless") {
    // Show main window
    let main_window = main.0.lock().unwrap();
    main_window.show().unwrap();
    main_window.set_always_on_top(true).unwrap();
    main_window.set_always_on_top(false).unwrap();
  }
}
