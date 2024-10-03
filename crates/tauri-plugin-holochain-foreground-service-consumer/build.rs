const COMMANDS: &[&str] = &[
  "request_install_app", "sign_zome_call"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}