#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

mod mobile;
mod error;
mod types;

pub use error::{Error, Result};
pub use types::*;

use mobile::HolochainForegroundServiceConsumer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-service APIs.
pub trait HolochainForegroundServiceConsumerExt<R: Runtime> {
  fn holochain_foreground_service_consumer(&self) -> &HolochainForegroundServiceConsumer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainForegroundServiceConsumerExt<R> for T {
  fn holochain_foreground_service_consumer(&self) -> &HolochainForegroundServiceConsumer<R> {
    self.state::<HolochainForegroundServiceConsumer<R>>().inner()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("holochain-service-consumer")
    .setup(|app, api| {
      let dialog = mobile::init(app, api)?;
      app.manage(dialog);
      Ok(())
    })
    .build()
}