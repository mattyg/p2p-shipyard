use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};
use crate::types::*;
use tracing::debug;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochainforegroundserviceconsumer";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain-foreground-service-consumer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<HolochainForegroundServiceConsumer<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainConsumerPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_holochain-foreground-service-consumer)?;
  Ok(HolochainForegroundServiceConsumer(handle))
}

/// Access to the holochain-foreground-service-consumer APIs.
pub struct HolochainForegroundServiceConsumer<R: Runtime>(pub PluginHandle<R>);

impl<R: Runtime> HolochainForegroundServiceConsumer<R> {
  pub fn is_app_installed(&self, app_id: &str)-> crate::Result<bool> {
    let res: IsAppInstalledResponse = self.0
      .run_mobile_plugin("isAppInstalled", AppIdRequestArgs { appId: app_id.to_string() })?;
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
      .run_mobile_plugin("appWebsocketAuth", AppIdRequestArgs { appId: app_id.to_string() })?
    )
  }

  pub fn sign_zome_call(&self, payload: SignZomeCallRequestArgs)-> crate::Result<SignZomeCallResponse> {
    Ok(
      self.0
      .run_mobile_plugin("signZomeCall", payload)?
    )
  }
}
