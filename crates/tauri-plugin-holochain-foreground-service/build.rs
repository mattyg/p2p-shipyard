const COMMANDS: &[&str] = &[
  "launch", "shutdown", "get_admin_port", "install_app", "create_app_websocket", "sign_zome_call"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}