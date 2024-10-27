use std::sync::Arc;

use sbd_server::{Config, SbdServer};
use url2::Url2;

pub async fn run_local_signal_service(local_ip: String, port: u16) -> std::io::Result<SbdServer> {
    let mut config = Config::default();

    config.bind = vec![format!("{local_ip}:{port}")];
    log::info!("Running local signal service {:?}", config);

    let sig_hnd = SbdServer::new(config.into()).await?;
    Ok(sig_hnd)
}
