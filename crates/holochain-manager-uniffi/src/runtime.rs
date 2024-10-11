use std::{collections::HashMap, sync::Arc};

use crate::{config::HolochainRuntimeFFIConfig, types::AppWebsocketAuthFFI};
use crate::error::HolochainRuntimeFFIError;
use crate::types::{AppInfoFFI, ZomeCallFFI, ZomeCallUnsignedTauriFFI};
use holochain_manager::{launch::launch_holochain_runtime, utils::vec_to_locked, HolochainRuntime};
use log::LevelFilter;
use android_logger::Config;
use holochain_types::{prelude::{AgentPubKey, AppBundle, MembraneProof, RoleName, SerializedBytes, UnsafeBytes}, websocket::AllowedOrigins};

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
            Config::default().with_max_level(LevelFilter::Warn),
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

    /// Is an app with the given installed_app_id installed on the conductor
    pub async fn is_app_installed(&self, installed_app_id: String) -> Result<bool, HolochainRuntimeFFIError> {
        self.runtime.is_app_installed(installed_app_id).await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(format!("{:?}", e)))
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

    /// Uninstall an app
    pub async fn uninstall_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.uninstall_app(app_id)
           .await
           .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Enable an installed app
    pub async fn enable_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.enable_app(app_id)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Disable an installed app
    pub async fn disable_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.disable_app(app_id)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }
    
    /// Get or create an app websocket with an authentication for the given app id
    pub async fn app_websocket_auth(&self, app_id: String) -> Result<AppWebsocketAuthFFI, HolochainRuntimeFFIError> {
        let app_websocket_auth = self.runtime.get_app_websocket_auth(&app_id, false, AllowedOrigins::Any)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(app_websocket_auth.into())
    }

    /// Sign a zome call
    pub async fn sign_zome_call(&self, zome_call_unsigned: ZomeCallUnsignedTauriFFI) -> Result<ZomeCallFFI, HolochainRuntimeFFIError> {
        let zome_call = self.runtime.sign_zome_call(zome_call_unsigned.into())
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(zome_call.into())
    }
}
