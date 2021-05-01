#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use aiarena_client_gui_backend_lib::actix_web;
use aiarena_client_gui_backend_lib::log::info;
use aiarena_client_gui_backend_lib::server::get_server;
use serde::Serialize;
use std::sync::mpsc;
use std::thread;

#[derive(Serialize)]
struct Reply {
  data: String,
}

#[actix_web::main]
async fn main() {
  ::std::env::set_var("RUST_LOG", "debug");
  env_logger::init();
  let (server_tx, server_rx) = mpsc::channel();

  //start actix web server in separate thread
  thread::spawn(move || {
    let sys = actix_web::rt::System::new("aiarena-client-gui-backend");

    let server = get_server("../backend/static/").unwrap();

    let _ = server_tx.send(server);
    let _ = sys.run();
  });

  let server = server_rx.recv().unwrap();
  let args: Vec<String> = std::env::args().collect();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::open_file_dialog,
      cmd::tauri_test,
      cmd::get_project_directory
    ])
    .run({
      let mut c = tauri::generate_context!();
      if args.contains(&"--headless".to_string()) {
        info!("Starting headless mode");
        for mut window in &mut c.config.tauri.windows {
          window.visible = false
        }
      }
      c
    })
    .unwrap();

  let _ = server.stop(true).await;
}
