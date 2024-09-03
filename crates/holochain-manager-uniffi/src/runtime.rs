use crate::config::HolochainRuntimeFFIConfig;
use crate::error::HolochainRuntimeFFIError;
use crate::types::AppInfoFFI;
use holochain_manager::{HolochainRuntime, launch::launch_holochain_runtime, utils::vec_to_locked};

#[derive(uniffi::Object)]
pub struct HolochainRuntimeFFI {
    runtime: HolochainRuntime,
}

#[uniffi::export]
impl HolochainRuntimeFFI {
    #[uniffi::constructor]
    pub async fn launch(passphrase: Vec<u8>, config: HolochainRuntimeFFIConfig) -> Result<Self, HolochainRuntimeFFIError> {
        let runtime = launch_holochain_runtime(
                vec_to_locked(passphrase).map_err(|e| HolochainRuntimeFFIError::IOError(e.to_string()))?,
                config.try_into()?
            )
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        
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
