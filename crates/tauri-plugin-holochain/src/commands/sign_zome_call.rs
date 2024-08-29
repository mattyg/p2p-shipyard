use holochain::{conductor::api::ZomeCall, prelude::ZomeCallUnsigned};
use holochain_types::prelude::Signature;
use lair_keystore_api::LairClient;
use tauri::{command, AppHandle, Runtime};

use crate::HolochainExt;

#[command]
pub(crate) async fn sign_zome_call<R: Runtime>(
    app_handle: AppHandle<R>,
    zome_call_unsigned: ZomeCallUnsigned,
) -> crate::Result<ZomeCall> {
    let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

    let signed_zome_call = sign_zome_call_with_client(
        zome_call_unsigned_converted,
        &app_handle
            .holochain()?
            .holochain_runtime
            .conductor_handle
            .keystore()
            .lair_client()
            .clone(),
    )
    .await?;

    Ok(signed_zome_call)
}

/// Signs an unsigned zome call with the given LairClient
pub async fn sign_zome_call_with_client(
    zome_call_unsigned: ZomeCallUnsigned,
    client: &LairClient,
) -> crate::Result<ZomeCall> {
    // sign the zome call
    let pub_key = zome_call_unsigned.provenance.clone();
    let mut pub_key_2 = [0; 32];
    pub_key_2.copy_from_slice(pub_key.get_raw_32());

    let data_to_sign = zome_call_unsigned.data_to_sign()?;

    let sig = client
        .sign_by_pub_key(pub_key_2.into(), None, data_to_sign)
        .await
        .map_err(|err| crate::Error::LairError(err))?;

    let signature = Signature(*sig.0);

    let signed_zome_call = ZomeCall {
        cell_id: zome_call_unsigned.cell_id,
        zome_name: zome_call_unsigned.zome_name,
        fn_name: zome_call_unsigned.fn_name,
        payload: zome_call_unsigned.payload,
        cap_secret: zome_call_unsigned.cap_secret,
        provenance: zome_call_unsigned.provenance,
        nonce: zome_call_unsigned.nonce,
        expires_at: zome_call_unsigned.expires_at,
        signature,
    };

    return Ok(signed_zome_call);
}
