use crate::error::HolochainRuntimeFFIConfig;
use crate::error::HolochainRuntimeFFIError;
use holochain_manager::{HolochainRuntime, launch::launch_holochain_runtime, utils::vec_to_locked};
use std::sync::Mutex;
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct HolochainRuntimeFFI {
    runtime: Arc<Mutex<Option<HolochainRuntime>>>,
}

#[uniffi::export]
impl HolochainRuntimeFFI {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn launch(&self, passphrase: Vec<u8>, config: HolochainRuntimeFFIConfig) -> Result<(), HolochainRuntimeFFIError> {
        let runtime = launch_holochain_runtime(
            vec_to_locked(passphrase).map_err(|e| HolochainRuntimeFFIError::IOError(e.to_string()))?,
            config.try_into()?
        )
        .await
        .map_err(|e| HolochainRuntimeFFIError::HolochainManagerError(e.to_string()))?;
    
        let mut runtime_lock = self.runtime.lock()?;
        *runtime_lock = Some(runtime);

        Ok(())
    }

}