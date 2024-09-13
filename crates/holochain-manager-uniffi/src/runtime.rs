use std::{collections::HashMap, sync::Arc};

use crate::config::HolochainRuntimeFFIConfig;
use crate::error::HolochainRuntimeFFIError;
use crate::types::AppInfoFFI;
use holochain_manager::{HolochainRuntime, launch::launch_holochain_runtime, utils::vec_to_locked};
use log::LevelFilter;
use android_logger::Config;
use holochain_types::prelude::{AgentPubKey, AppBundle, MembraneProof, RoleName, SerializedBytes, UnsafeBytes};

#[derive(uniffi::Object)]
pub struct HolochainRuntimeFFI {
    runtime: HolochainRuntime,
}

#[uniffi::export(async_runtime = "tokio")]
impl HolochainRuntimeFFI {

    /// Start the holochain conductor
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

    /// Shutdown the holochain conductor
    pub async fn shutdown(&self) -> Result<(), HolochainRuntimeFFIError> {
        self.runtime.shutdown().await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Get an admin port on the conductor
    pub fn get_admin_port(&self) -> u16 {
        self.runtime.admin_port
    }

    /// List apps installed on the conductor
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

    /// Install an app
    pub async fn install_app(
        &self,
        app_id: String,
        app_bundle_bytes: Vec<u8>,
        membrane_proofs: HashMap<String, Vec<u8>>,
        agent: Option<Vec<u8>>,
        network_seed: Option<String>
    ) -> Result<(), HolochainRuntimeFFIError> {
        let agent = agent.and_then(|a| 
                Some(AgentPubKey::from_raw_39(a))
            ).transpose()
            .map_err(|e| HolochainRuntimeFFIError::Infallible(e.to_string()))?;
        let app_bundle = AppBundle::decode(app_bundle_bytes.as_slice()).map_err(|e| HolochainRuntimeFFIError::Infallible(e.to_string()))?;
        let mut membrane_proofs_typed: HashMap<RoleName, MembraneProof> = HashMap::new();
        for (k, v) in membrane_proofs {
            let proof = SerializedBytes::try_from(UnsafeBytes::from(v))
                .map_err(|e| HolochainRuntimeFFIError::Infallible(e.to_string()))?;
            membrane_proofs_typed.insert(k, Arc::new(proof));
        }
        
        self.runtime.install_app(app_id, app_bundle, membrane_proofs_typed, agent, network_seed)
           .await
           .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }
}
