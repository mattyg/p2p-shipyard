#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, RunEvent
};
use std::collections::HashMap;

mod mobile;
mod error;
mod types;

pub use error::{Error, Result};
pub use types::*;

use mobile::HolochainForegroundServiceConsumer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-foreground-service APIs.
pub trait HolochainForegroundServiceConsumerExt<R: Runtime> {
  fn holochain_foreground_service_consumer(&self) -> &HolochainForegroundServiceConsumer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainForegroundServiceConsumerExt<R> for T {
  fn holochain_foreground_service_consumer(&self) -> &HolochainForegroundServiceConsumer<R> {
    self.state::<HolochainForegroundServiceConsumer<R>>().inner()
  }
}

pub fn init<R: Runtime>(
//  app_id: String, app_bundle_bytes: Vec<u8>, membrane_proofs: HashMap<String, Vec<u8>>, agent: Option<Vec<u8>>, network_seed: Option<String>
) -> TauriPlugin<R> {
  Builder::new("holochain-foreground-service-consumer")
    .setup(|app, api| {
      let dialog = mobile::init(app, api)?;
      app.manage(dialog);
      Ok(())
    })
    /*
    .on_event(move |app, event| {
      if let RunEvent::Ready = event {
        println!("RunEvent::Ready");

        let plugin = app.app_handle().holochain_foreground_service_consumer();

        // Check if app with this app id is installed
        let is_app_installed = plugin.is_app_installed(app_id.clone().as_str())
              .expect("Could not determine if app is installed");
          println!("is_app_installed: {}", is_app_installed);
          
          if !is_app_installed {
              // Install app if not installed
              println!("installing app");
              let _ = plugin.install_app(InstallAppRequestArgs {
                  app_id: app_id.clone(),
                  app_bundle_bytes: app_bundle_bytes.clone(),
                  membrane_proofs: membrane_proofs.clone(),
                  agent: agent.clone(),
                  network_seed: network_seed.clone(),
              });
          }

          // Get or create app websocket for this app
          let auth = plugin.app_websocket_auth(app_id.clone().as_str()).expect("Could not create app websocket auth");
          println!("app_websocket_auth {:?}", auth);
      }
    })
    */
    .build()
}