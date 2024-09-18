use holochain_conductor_api::{AppInfo, ZomeCall};
use holochain_manager::{commands::ZomeCallUnsignedTauri, AppWebsocketAuth};
use holochain_types::{dna::{hash_type::{Agent, Dna}, HoloHash}, prelude::{CapSecret, CellId, ExternIO, FunctionName, Timestamp, ZomeName}};

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

#[derive(uniffi::Record)]
pub struct CellIdFFI {
  pub dna_hash: Vec<u8>,
  pub agent_pub_key: Vec<u8>,
}

impl From<CellId> for CellIdFFI {
  fn from(value: CellId) -> Self {
    Self {
      dna_hash: value.dna_hash().get_raw_39().to_vec(),
      agent_pub_key: value.agent_pubkey().get_raw_39().to_vec(),
    }
  }
}

impl Into<CellId> for CellIdFFI {
  fn into(self) -> CellId {
    CellId::new(
      HoloHash::<Dna>::from_raw_39(self.dna_hash).unwrap(), 
      HoloHash::<Agent>::from_raw_39(self.agent_pub_key).unwrap()
    )
  }
}

#[derive(uniffi::Record)]
pub struct ZomeCallUnsignedTauriFFI {
  pub provenance: Vec<u8>,
  pub cell_id: CellIdFFI,
  pub zome_name: String,
  pub fn_name: String,
  pub cap_secret: Option<Vec<u8>>,
  pub payload: Vec<u8>,
  pub nonce: Vec<u8>,
  pub expires_at: i64,
}

impl From<ZomeCallUnsignedTauri> for ZomeCallUnsignedTauriFFI {
  fn from(value: ZomeCallUnsignedTauri) -> Self {
      Self {
        provenance: value.provenance.get_raw_39().to_vec(),
        cell_id: value.cell_id.into(),
        zome_name: value.zome_name.0.to_string(),
        fn_name: value.fn_name.into(),
        cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
        payload: value.payload.into(),
        nonce: value.nonce.into(),
        expires_at: value.expires_at.0
      }
  }
}

impl Into<ZomeCallUnsignedTauri> for ZomeCallUnsignedTauriFFI {
  fn into(self) -> ZomeCallUnsignedTauri {
    let nonce: [u8; 32] = self.nonce.as_slice().try_into().unwrap();
    let cap_secret: Option<[u8; 64]> = self.cap_secret.map(|s| s.as_slice().try_into().unwrap());

    ZomeCallUnsignedTauri {
        provenance: HoloHash::<Agent>::from_raw_39(self.provenance).unwrap(),
        cell_id: self.cell_id.into(),
        zome_name: ZomeName::new(self.zome_name),
        fn_name: FunctionName::new(self.fn_name),
        cap_secret: cap_secret.map(|s| CapSecret::from(s)),
        payload: ExternIO::from(self.payload),
        nonce,
        expires_at: Timestamp(self.expires_at),
      }
  }
}

#[derive(uniffi::Record)]
pub struct ZomeCallFFI {
    pub cell_id: CellIdFFI,
    pub zome_name: String,
    pub fn_name: String,
    pub payload: Vec<u8>,
    pub cap_secret: Option<Vec<u8>>,
    pub provenance: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCall> for ZomeCallFFI {
  fn from(value: ZomeCall) -> Self {
      Self {
        cell_id: value.cell_id.into(),
        zome_name: value.zome_name.0.to_string(),
        fn_name: value.fn_name.into(),
        payload: value.payload.into(),
        cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
        provenance: value.provenance.get_raw_39().to_vec(),
        signature: value.signature.0.to_vec(),
        nonce: value.nonce.into_inner().to_vec(),
        expires_at: value.expires_at.0
      }
  }
}
