use std::collections::HashMap;
use tauri_plugin_holochain_foreground_service_consumer::{HolochainForegroundServiceConsumerExt, InstallAppRequestArgs};
use tracing::debug;
use tauri::Manager;

const APP_ID: &str = "tauri-app";
const HAPP_PATH: &str = "../../workdir/tauri-app.happ";
const SIGNAL_URL: &str = "wss://signal.holo.host";
const BOOTSTRAP_URL: &str = "https://bootstrap.holo.host";


fn setup_app_script() -> String {
    let prefix = r#"
        async function n(n,e={},a){return window.__TAURI_INTERNALS__.invoke(n,e,a)}"function"==typeof SuppressedError&&SuppressedError,window.invokeSetupApp=async function(e,a,o,r,i){await async function(e){return await n("plugin:holochain-foreground-service-consumer|is_app_installed",{appId:e}).then((n=>n.installed))}(e)||await async function(e){return await n("plugin:holochain-foreground-service-consumer|install_app",e)}({appId:e,appBundleBytes:a,membraneProofs:o,agent:r,networkSeed:i}),await async function(e){return await n("plugin:holochain-foreground-service-consumer|app_websocket_auth",{appId:e})}(e)};
    "#;

    let script = format!(
        r#"
            {}
            function setupApp() {{
                window.invokeSetupApp("{}", new Uint8Array([1,2,3]), new Map(), undefined, "{}");
            }}
            setupApp();
        "#,
        prefix,
        //include_bytes!("../../src/forum.happ").to_vec(), 
        APP_ID.to_string(), 
        "dev-2024-10-06"
    );
    println!("setup_app_script {}", script);

    script
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_holochain_foreground_service_consumer::init(
            APP_ID.to_string(),
            include_bytes!("../../src/forum.happ").to_vec(),
            HashMap::new(),
            None,
            Some("dev-2024-10-04".to_string())
        ))
        .append_invoke_initialization_script(setup_app_script().as_str())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
