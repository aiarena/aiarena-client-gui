#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use aiarena_client_gui_backend_lib::actix_web_static_files;
use aiarena_client_gui_backend_lib::routes::*;
use aiarena_client_gui_backend_lib::{actix_web, web, Handlebars, HttpServer, OpenApiExt};
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

    let mut handlebars = Handlebars::new();
    handlebars
      .register_templates_directory(".html", "../backend/static/")
      .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let server = HttpServer::new(move || {
      let generated = generate();
      App::new()
        .app_data(handlebars_ref.clone())
        .service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
        .service(actix_web::web::resource("/watch").route(actix_web::web::get().to(watch)))
        .service(actix_web::web::resource("/settings").route(actix_web::web::get().to(settings)))
        .service(actix_web_static_files::ResourceFiles::new(
          "/static", generated,
        ))
        .wrap_api()
        .service(web::resource("/get_maps").route(web::get().to(get_maps)))
        .service(web::resource("/get_bots").route(web::get().to(get_bots)))
        .service(web::resource("/get_settings").route(web::get().to(get_settings)))
        .service(web::resource("/get_arena_bots").route(web::get().to(get_arena_bots)))
        .service(web::resource("/run_games").route(web::post().to(run_games)))
        .service(web::resource("/handle_data").route(web::post().to(handle_data)))
        .service(web::resource("/get_results").route(web::get().to(get_results)))
        .with_json_spec_at("/api")
        .build()
    })
    .bind("127.0.0.1:8082")
    .unwrap()
    .run();

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
