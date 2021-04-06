#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use std::thread;
use std::sync::mpsc;
use aiarena_client_gui_backend_lib::routes::*;
use aiarena_client_gui_backend_lib::{HttpServer, web, actix_web, Handlebars, OpenApiExt};
use aiarena_client_gui_backend_lib::{actix_web_static_files, actix};
use web_view::*;

#[actix_web::main]
async fn main() {
    let (server_tx, server_rx) = mpsc::channel();

    //start actix web server in separate thread
    thread::spawn(move  ||{
        let sys = actix_web::rt::System::new("aiarena-client-gui-backend");

        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".html", "./backend/static/")
            .unwrap();
        let handlebars_ref = web::Data::new(handlebars);

        let server = HttpServer::new(move || {
            let generated = generate();
            App::new()
                .app_data(handlebars_ref.clone())
                .service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
                .service(actix_web::web::resource("/watch").route(actix_web::web::get().to(watch)))
                .service(
                    actix_web::web::resource("/settings").route(actix_web::web::get().to(settings)),
                )
                .service(actix_web_static_files::ResourceFiles::new(
                    "/static", generated,
                ))
                .wrap_api()
                .service(web::resource("/get_maps").route(web::post().to(get_maps)))
                .service(web::resource("/get_bots").route(web::post().to(get_bots)))
                .service(web::resource("/get_settings").route(web::post().to(get_settings)))
                .service(web::resource("/get_arena_bots").route(web::post().to(get_arena_bots)))
                .service(web::resource("/run_games").route(web::post().to(run_games)))
                .service(web::resource("/handle_data").route(web::post().to(handle_data)))
                .with_json_spec_at("/api")
                .build()
        })
            .bind("127.0.0.1:8080").unwrap()
            .run();


        let _ = server_tx.send(server);
        let _ = sys.run();
    });

    let server = server_rx.recv().unwrap();

    let html = format!(
        r#"
		<!doctype html>
		<html>
			<head>
            <script>
            if (typeof Object.assign != 'function') {{
                Object.assign = function(target) {{
                  'use strict';
                  if (target == null) {{
                    throw new TypeError('Cannot convert undefined or null to object');
                  }}

                  target = Object(target);
                  for (var index = 1; index < arguments.length; index++) {{
                    var source = arguments[index];
                    if (source != null) {{
                      for (var key in source) {{
                        if (Object.prototype.hasOwnProperty.call(source, key)) {{
                          target[key] = source[key];
                        }}
                      }}
                    }}
                  }}
                  return target;
                }};
              }}</script>
				{styles}
			</head>
			<body>
				<!--[if lt IE 9]>
				<div class="ie-upgrade-container">
					<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
					<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
				</div>
				<![endif]-->
				<!--[if gte IE 9 | !IE ]> <!-->
				<div id="app"></div>
				{scripts}
				<![endif]-->
			</body>
		</html>
		"#,
        styles = inline_style(include_str!("../frontend/build/app.css")),
        scripts = inline_script(include_str!("../frontend/build/app.js"))
    );

    let mut webview = web_view::builder()
        .title("Rust Todo App")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(vec![])
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            let tasks_len = {
                let tasks = webview.user_data_mut();

                match serde_json::from_str(arg).unwrap() {
                    Init => (),
                    Log { text } => println!("{}", text),
                    AddTask { name } => tasks.push(Task { name, done: false }),
                    MarkTask { index, done } => tasks[index].done = done,
                    ClearDoneTasks => tasks.retain(|t| !t.done),
                }

                tasks.len()
            };

            webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks_len))?;
            render(webview)
        })
        .build()
        .unwrap();

    webview.set_color((156, 39, 176));

    let res = webview.run().unwrap();

    println!("final state: {:?}", res);
    // gracefully shutdown actix web server
    let _ = server.stop(true).await;
}

fn render(webview: &mut WebView<Vec<Task>>) -> WVResult {
    let render_tasks = {
        let tasks = webview.user_data();
        println!("{:#?}", tasks);
        format!("app.fromRust({})", serde_json::to_string(tasks).unwrap())
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
