#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use aiarena_client_gui_backend_lib::actix_web;
use aiarena_client_gui_backend_lib::server::get_server;
use std::sync::mpsc;
use std::thread;

#[actix_web::main]
async fn main() {
  ::std::env::set_var("RUST_LOG", "rust_ac=trace");
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
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
  let _ = server.stop(true).await;
}
