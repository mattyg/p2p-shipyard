use clap::Parser;
use holochain_types::{dna::AgentPubKey, prelude::AppBundle};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_holochain::{HolochainExt, HolochainPluginConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the file tree to modify.
    pub happ_path: PathBuf,

    /// The bundle identifier for the Tauri app
    #[clap(long)]
    pub ui_port: Option<String>,

    /// The bundle identifier for the Tauri app
    #[clap(long)]
    pub signal_url: String,

    /// The bundle identifier for the Tauri app
    #[clap(long)]
    pub bootstrap_url: String,
}

const APP_ID: &'static str = "example";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args = Args::parse();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_holochain::async_init(
            vec_to_locked(vec![]).expect("Can't build passphrase"),
            HolochainPluginConfig {
                signal_url: url2::parse(args.signal_url),
                bootstrap_url: bootstrap_url(),
                holochain_dir: holochain_dir(),
            },
        ))
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                setup(handle.clone()).await.expect("Failed to setup");

                handle
                    .holochain()
                    .expect("Failed to get holochain")
                    .main_window_builder(String::from("main"), false, Some(APP_ID.into()), None)
                    .await
                    .expect("Failed to build window")
                    .build()
                    .expect("Failed to open main window");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Very simple setup for now:
// - On app start, list installed apps:
//   - If there are no apps installed, this is the first time the app is opened: install our hApp
//   - If there **are** apps:
//     - Check if it's necessary to update the coordinators for our hApp
//       - And do so if it is
//
// You can modify this function to suit your needs if they become more complex
async fn setup(
    handle: AppHandle,
    app_bundle_path: PathBuf,
    agent_pub_key: AgentPubKey,
) -> anyhow::Result<()> {
    let admin_ws = handle.holochain()?.admin_websocket().await?;

    let installed_apps = admin_ws
        .list_apps(None)
        .await
        .map_err(|err| tauri_plugin_holochain::Error::ConductorApiError(err))?;

    handle
        .holochain()?
        .install_app(String::from(APP_ID), example_happ(), HashMap::new(), None)
        .await?;

    Ok(())
}
fn main() {
    println!("Hello, world!");
}
