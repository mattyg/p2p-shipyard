use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};
use crate::types::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochain_service_consumer";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain-service-consumer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<HolochainServiceConsumer<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainConsumerPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_holochain-service-consumer)?;
  Ok(HolochainServiceConsumer(handle))
}

/// Access to the holochain-service-consumer APIs.
pub struct HolochainServiceConsumer<R: Runtime>(pub PluginHandle<R>);

impl<R: Runtime> HolochainServiceConsumer<R> {
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

  pub fn app_websocket_auth(&self, app_id: &str)-> crate::Result<AppWebsocketAuthResponse> {
    Ok(
      self.0
      .run_mobile_plugin("appWebsocketAuth", AppIdRequestArgs { app_id: app_id.to_string() })?
    )
  }

  pub fn sign_zome_call(&self, payload: SignZomeCallRequestArgs)-> crate::Result<SignZomeCallResponse> {
    Ok(
      self.0
      .run_mobile_plugin("signZomeCall", payload)?
    )
  }
}
