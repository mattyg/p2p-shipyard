#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod mobile;
mod error;
mod types;

pub use error::{Error, Result};
pub use types::*;

use mobile::HolochainForegroundService;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-foreground-service APIs.
pub trait HolochainForegroundServiceExt<R: Runtime> {
  fn holochain_foreground_service(&self) -> &HolochainForegroundService<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainForegroundServiceExt<R> for T {
  fn holochain_foreground_service(&self) -> &HolochainForegroundService<R> {
    self.state::<HolochainForegroundService<R>>().inner()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("holochain-foreground-service")
    .setup(|app, api| {
      let dialog = mobile::init(app, api)?;
      app.manage(dialog);
      Ok(())
    })
    .build()
}