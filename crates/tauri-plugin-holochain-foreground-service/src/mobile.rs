use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::types::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochainforegroundservice";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain-foreground-service);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<HolochainForegroundService<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_holochain-foreground-service)?;
  Ok(HolochainForegroundService(handle))
}

/// Access to the holochain-foreground-service APIs.
pub struct HolochainForegroundService<R: Runtime>(pub PluginHandle<R>);

// TODO: replace Blank request & responses with actual types
impl<R: Runtime> HolochainForegroundService<R> {
  pub fn launch(&self, payload: Blank) -> crate::Result<()> {
    self.0
      .run_mobile_plugin("launch", payload)
      .map_err(Into::into)
  }
  
  pub fn shutdown(&self, payload: Blank)-> crate::Result<()> {
    self.0
      .run_mobile_plugin("shutdown", payload)
      .map_err(Into::into)
  }

  pub fn get_admin_port(&self, payload: Blank)-> crate::Result<()> {
    self.0
      .run_mobile_plugin("getAdminPort", payload)
      .map_err(Into::into)
  }

  pub fn install_app(&self, payload: Blank)-> crate::Result<()> {
    self.0
      .run_mobile_plugin("installApp", payload)
      .map_err(Into::into)
  }

  pub fn create_app_websocket(&self, payload: Blank)-> crate::Result<()> {
    self.0
      .run_mobile_plugin("createAppWebsocket", payload)
      .map_err(Into::into)
  }

  pub fn sign_zome_call(&self, payload: Blank)-> crate::Result<()> {
    self.0
      .run_mobile_plugin("signZomeCall", payload)
      .map_err(Into::into)
  }
}
