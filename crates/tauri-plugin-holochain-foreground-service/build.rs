const COMMANDS: &[&str] = &[
  "launch", "shutdown", "get_admin_port", "install_app", "list_installed_apps", "create_app_websocket", "sign_zome_call", "app_websocket_auth"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}