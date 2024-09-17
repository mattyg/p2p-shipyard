use holochain_conductor_api::AppInfo;
use holochain_manager::AppWebsocketAuth;

#[derive(uniffi::Record)]
pub struct AppInfoFFI {
  /// The unique identifier for an installed app in this conductor
  pub installed_app_id: String,
}

impl From<AppInfo> for AppInfoFFI {
  fn from(value: AppInfo) -> Self {
      Self {
        installed_app_id: value.installed_app_id,
      }
  }
}

#[derive(uniffi::Record)]
pub struct AppWebsocketAuthFFI {
    pub app_id: String,
    pub port: u16,
    pub token: Vec<u8>,
}

impl From<AppWebsocketAuth> for AppWebsocketAuthFFI {
  fn from(value: AppWebsocketAuth) -> Self {
      Self {
        app_id: value.app_id,
        port: value.app_websocket_port,
        token: value.token,
      }
  }
}
