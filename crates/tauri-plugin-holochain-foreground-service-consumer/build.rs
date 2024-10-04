const COMMANDS: &[&str] = &[
  "install_app", "app_websocket_auth", "sign_zome_call"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}