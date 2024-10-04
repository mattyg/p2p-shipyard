use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

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
  pub fn install_app(&self, payload: HolochainRequest)-> crate::Result<HolochainResponse> {
    self.0
      .run_mobile_plugin("installApp", payload)
      .map_err(Into::into)
  }

  pub fn create_app_websocket(&self, payload: HolochainRequest)-> crate::Result<HolochainResponse> {
    self.0
      .run_mobile_plugin("createAppWebsocket", payload)
      .map_err(Into::into)
  }

  pub fn sign_zome_call(&self, payload: HolochainRequest)-> crate::Result<HolochainResponse> {
    self.0
      .run_mobile_plugin("signZomeCall", payload)
      .map_err(Into::into)
  }
}
