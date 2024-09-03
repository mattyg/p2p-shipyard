use crate::config::HolochainRuntimeFFIConfig;
use crate::error::HolochainRuntimeFFIError;
use crate::types::AppInfoFFI;
use holochain_manager::{HolochainRuntime, launch::launch_holochain_runtime, utils::vec_to_locked};
use log::LevelFilter;
use android_logger::Config;


#[derive(uniffi::Object)]
pub struct HolochainRuntimeFFI {
    runtime: HolochainRuntime,
}

#[uniffi::export]
impl HolochainRuntimeFFI {
    #[uniffi::constructor]
    pub async fn launch(passphrase: Vec<u8>, config: HolochainRuntimeFFIConfig) -> Result<Self, HolochainRuntimeFFIError> {
        android_logger::init_once(
            Config::default().with_max_level(LevelFilter::Trace),
        );
        debug!("HolochainRuntimeFFI 1");
        
        let passphrase_bufread =  vec_to_locked(passphrase).map_err(|e| HolochainRuntimeFFIError::IOError(e.to_string()))?;
        debug!("HolochainRuntimeFFI 2");

        let runtime = launch_holochain_runtime(
                passphrase_bufread,
                config.try_into()?
            )
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        debug!("HolochainRuntimeFFI 3");
        
        Ok(HolochainRuntimeFFI {
            runtime
        })
    }

    pub async fn list_installed_apps(&self) -> Result<Vec<AppInfoFFI>, HolochainRuntimeFFIError> {
        let apps = self.runtime.admin_websocket().await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?
            .list_apps(None).await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(format!("{:?}", e)))?
            .into_iter()
            .map(|a| a.into())
            .collect();
        
        Ok(apps)
    }

    pub fn get_admin_port(&self) -> u16 {
        self.runtime.admin_port
    }
}
