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
use std::thread::sleep;
use std::time::Duration;
use tauri::{LogicalPosition, Manager, Position};

#[derive(Serialize)]
struct Reply {
  data: String,
}

#[actix_web::main]
async fn main() {
  let args: Vec<String> = std::env::args().collect();
  let mut log_arg = None;
  if let Some((index, _)) = args.iter().enumerate().find(|&(_, x)| x == "--log") {
    log_arg = args.get(index + 1);
  }
  if let Some(arg) = log_arg {
    ::std::env::set_var("RUST_LOG", arg);
  } else if ::std::env::var_os("RUST_LOG").is_none() {
    ::std::env::set_var("RUST_LOG", "info");
  }
  env_logger::init();
  let (server_tx, server_rx) = mpsc::channel();

  //start actix web server in separate thread
  thread::spawn(move || {
    let sys = actix_web::rt::System::new("aiarena-client-gui-backend");

    let server = get_server().unwrap();

    let _ = server_tx.send(server);
    let _ = sys.run();
  });

  let server = server_rx.recv().unwrap();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::open_file_dialog,
      cmd::tauri_test,
      cmd::get_project_directory,
      cmd::open_directory,
      cmd::restart_app_with_logs,
      cmd::get_debug_logs_directory
    ])
    .setup(move |app| {
      if args.contains(&"--headless".to_string()) {
        info!("Headless mode is not currently implemented");
        // for window in app.config().tauri.windows{
        //   window.visible =false;
        // }
      } else {
        let splashscreen = app.get_window("splashscreen").unwrap();
        let main = app.get_window("main").unwrap();
        let default_screen_size = (1920.0, 1080.0);
        let splash_screen_size = (400.0, 200.0);
        let center = LogicalPosition {
          x: (default_screen_size.0 / 2.0) - (splash_screen_size.0 / 2.0) as f64,
          y: (default_screen_size.1 / 2.0) - (splash_screen_size.1 / 2.0) as f64,
        };
        splashscreen
          .set_position(Position::Logical(center))
          .unwrap();

        tauri::async_runtime::spawn(async move {
          sleep(Duration::from_secs(2));
          splashscreen.close().unwrap();
          main.show().unwrap();
          main.set_always_on_top(true).unwrap();
          main.set_always_on_top(false).unwrap();
        });
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap();

  let _ = server.stop(true).await;
}
