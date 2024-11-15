use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::types::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochainforegroundservice";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain-service);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<HolochainForegroundService<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_holochain-service)?;
  Ok(HolochainForegroundService(handle))
}

/// Access to the holochain-service APIs.
pub struct HolochainForegroundService<R: Runtime>(pub PluginHandle<R>);

// TODO: replace Blank request & responses with actual types
impl<R: Runtime> HolochainForegroundService<R> {
  pub fn launch(&self) -> crate::Result<()> {
    Ok(
      self.0
      .run_mobile_plugin("launch", Blank {})?
    )
  }
  
  pub fn shutdown(&self)-> crate::Result<()> {
    Ok(
      self.0
      .run_mobile_plugin("shutdown", Blank {})?
    )
  }

  pub fn get_admin_port(&self)-> crate::Result<()> {
    Ok(
      self.0
      .run_mobile_plugin("getAdminPort", Blank {})?
    )
  }

  pub fn is_app_installed(&self, app_id: &str)-> crate::Result<bool> {
    let res: IsAppInstalledResponse = self.0
      .run_mobile_plugin("isAppInstalled", AppIdRequestArgs { app_id: app_id.to_string() })?;

      Ok(res.installed)
  }

  pub fn install_app(&self, payload: InstallAppRequestArgs)-> crate::Result<()> {
    Ok(
      self.0
        .run_mobile_plugin("installApp", payload)?
    )
  }

  pub fn app_websocket_auth(&self, app_id: String)-> crate::Result<AppWebsocketAuthResponse> {
    Ok(
      self.0
      .run_mobile_plugin("appWebsocketAuth", AppIdRequestArgs { app_id })?
    )
  }

  pub fn sign_zome_call(&self, payload: SignZomeCallRequestArgs)-> crate::Result<SignZomeCallResponse> {
    Ok(
      self.0
      .run_mobile_plugin("signZomeCall", payload)?
    )
  }
}
