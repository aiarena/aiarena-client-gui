use aiarena_client_gui_backend_lib::actix_web_static_files::resource_dir;

fn main() {
  resource_dir("../backend/static").build().unwrap();
  tauri_build::build();
}
