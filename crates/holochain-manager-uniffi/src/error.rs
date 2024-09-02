
use holochain_manager::{HolochainManagerConfig, WANNetworkConfig};
use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::PoisonError;
use url2::{Url2, Url2Error};

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum HolochainRuntimeFFIError {
    #[error("Holochain Manager Error: {0}")]
    HolochainManagerError(String),

    #[error(transparent)]
    HolochainRuntimeFFIConfigError(#[from] HolochainRuntimeFFIConfigError),

    #[error("Mutex poisoned")]
    PoisonError,

    #[error("IO Error: {0}")]
    IOError(String)
}
impl<T> From<PoisonError<T>> for HolochainRuntimeFFIError {
    fn from(_err: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}


#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum HolochainRuntimeFFIConfigError {
    #[error(transparent)]
    Url2Error(#[from] Url2Error),

    #[error(transparent)]
    Infallible(#[from] Infallible),
}

#[derive(uniffi::Record)]
pub struct HolochainRuntimeFFIConfig {
    /// URL of bootstrap server
    bootstrap_url: String,

    /// URL of signal server
    signal_url: String,

    /// Path to directory where conductor data will be stored
    holochain_dir: String,
}

impl TryInto<HolochainManagerConfig> for HolochainRuntimeFFIConfig {
    type Error = HolochainRuntimeFFIConfigError;
    fn try_into(self) -> Result<HolochainManagerConfig, Self::Error> {
        Ok(HolochainManagerConfig {
            wan_network_config: Some(WANNetworkConfig {
                bootstrap_url: Url2::try_parse(self.bootstrap_url)?,
                signal_url: Url2::try_parse(self.signal_url)?,
            }),
            holochain_dir: PathBuf::from_str(self.holochain_dir.as_str())?,
        })
    }
}
