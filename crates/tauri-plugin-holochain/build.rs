const COMMANDS: &[&str] = &[
    "sign_zome_call",
    "get_locales",
    "open_app",
    "list_apps",
    "is_holochain_ready",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        // .android_path("android")
        // .ios_path("ios")
        .build();

    // Add an entitlement on macOS builds to ensure
    // the app is allowed to use just-in-time wasm compilation
    #[cfg(target_os = "macos")]
    {
        tauri_plugin::mobile::update_entitlements(|entitlements| {
            entitlements.insert(
                "com.apple.security.cs.allow-jit".into(),
                true
            );
            entitlements.insert(
                "com.apple.security.cs.allow-unsigned-executable-memory".into(),
                true
            );
            entitlements.insert(
                "com.apple.security.cs.disable-library-validation".into(),
                true
            );
        })
        .expect("failed to update entitlements");
    }
}
