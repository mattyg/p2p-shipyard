use std::{sync::Arc, time::Duration};

use async_std::sync::Mutex;
use tx5_signal_srv::SrvHnd;
use url2::url2;

use hc_seed_bundle::dependencies::sodoken::BufRead;
use holochain::conductor::Conductor;
use holochain_client::AdminWebsocket;

use crate::{
    filesystem::FileSystem, launch::signal::{can_connect_to_signal_server, run_local_signal_service}, GossipArcClamp, HolochainRuntime, HolochainRuntimeConfig
};

mod mdns;
mod signal;
mod config;
use mdns::spawn_mdns_bootstrap;

// pub static RUNNING_HOLOCHAIN: RwLock<Option<RunningHolochainInfo>> = RwLock::const_new(None);

/// Launch the holochain conductor in the background
pub(crate) async fn launch_holochain_runtime(
    passphrase: BufRead,
    config: HolochainRuntimeConfig,
) -> crate::error::Result<HolochainRuntime> {
    // let mut lock = RUNNING_HOLOCHAIN.write().await;

    // if let Some(info) = lock.to_owned() {
    //     return Ok(info);
    // }

    let filesystem = FileSystem::new(config.holochain_dir).await?;
    let admin_port = if let Some(admin_port) = config.admin_port {
        admin_port
    } else {
        portpicker::pick_unused_port().expect("No ports free")
    };

    let mut maybe_local_signal_server: Option<(url2::Url2, SrvHnd)> = None;

    let run_local_signal_server = if let Some(network_config) = &config.wan_network_config {
        if let Err(err) = can_connect_to_signal_server(network_config.signal_url.clone()).await {
            log::warn!("Error connecting with the WAN signal server: {err:?}");
            config.fallback_to_lan_only
        } else {
            false
        }
    } else {
        true
    };

    if run_local_signal_server {
        let my_local_ip = local_ip_address::local_ip().expect("Could not get local ip address");
        let port = portpicker::pick_unused_port().expect("No ports free");
        let signal_handle = run_local_signal_service(my_local_ip.to_string(), port).await?;

        let local_signal_url = url2!("ws://{my_local_ip}:{port}");

        maybe_local_signal_server = Some((local_signal_url.clone(), signal_handle));
    }


    let config = config::conductor_config(
        &filesystem,
        admin_port,
        filesystem.keystore_dir().into(),
        config.wan_network_config,
        maybe_local_signal_server.as_ref().map(|s| s.0.clone()),
        config.gossip_arc_clamp.map(|n| match n {
            GossipArcClamp::Full => "full".to_string(),
            GossipArcClamp::Empty => "empty".to_string(),
        }),
    );

    let conductor_handle = Conductor::builder()
        .config(config)
        .passphrase(Some(passphrase))
        .build()
        .await?;

    wait_until_admin_ws_is_available(admin_port).await?;
    log::info!("Connected to the admin websocket");

    spawn_mdns_bootstrap(admin_port).await?;

    // *lock = Some(info.clone());

    Ok(HolochainRuntime {
        filesystem,
        apps_websockets_auths: Arc::new(Mutex::new(Vec::new())),
        admin_port,
        conductor_handle,
        _signal_handle: maybe_local_signal_server.map(|s| s.1),
    })
}

pub async fn wait_until_admin_ws_is_available(admin_port: u16) -> crate::Result<()> {
    let mut retry_count = 0;
    loop {
        if let Err(err) = AdminWebsocket::connect(format!("localhost:{}", admin_port)).await {
            log::error!("Could not connect to the admin interface: {}", err);
        } else {
            break;
        }
        async_std::task::sleep(Duration::from_millis(200)).await;

        retry_count += 1;
        if retry_count == 200 {
            return Err(crate::Error::AdminWebsocketError(
                "Can't connect to holochain".to_string(),
            ));
        }
    }
    Ok(())
}


// fn read_config(config_path: &std::path::Path) -> crate::Result<LairServerConfig> {
//     let bytes = std::fs::read(config_path)?;

//     let config =
//         LairServerConfigInner::from_bytes(&bytes).map_err(|err| crate::Error::LairError(err))?;

//     if let Err(e) = std::fs::read(config.clone().pid_file) {
//         // Workaround xcode different containers
//         std::fs::remove_dir_all(config_path.parent().unwrap())?;
//         std::fs::create_dir_all(config_path.parent().unwrap())?;
//         return Err(e)?;
//     }

//     Ok(Arc::new(config))
// }

// /// Spawn an in-process keystore backed by lair_keystore.
// pub async fn spawn_lair_keystore_in_proc(
//     config_path: std::path::PathBuf,
//     passphrase: BufRead,
// ) -> LairResult<MetaLairClient> {
//     // return Ok(spawn_test_keystore().await?);

//     let config = get_config(&config_path, passphrase.clone()).await?;
//     let connection_url = config.connection_url.clone();

//     // rather than using the in-proc server directly,
//     // use the actual standalone server so we get the pid-checks, etc
//     let mut server = StandaloneServer::new(config).await?;

//     server.run(passphrase.clone()).await?; // 3 seconds

//     // just incase a Drop gets impld at some point...
//     std::mem::forget(server);

//     // now, just connect to it : )
//     let k = spawn_lair_keystore(connection_url.into(), passphrase).await?; // 2 seconds
//     Ok(k)
// }

// pub async fn get_config(
//     config_path: &std::path::Path,
//     passphrase: BufRead,
// ) -> LairResult<LairServerConfig> {
//     match read_config(config_path) {
//         Ok(config) => Ok(config),
//         Err(_) => write_config(config_path, passphrase).await,
//     }
// }

// pub async fn write_config(
//     config_path: &std::path::Path,
//     passphrase: BufRead,
// ) -> LairResult<LairServerConfig> {
//     let lair_root = config_path
//         .parent()
//         .ok_or_else(|| one_err::OneErr::from("InvalidLairConfigDir"))?;

//     tokio::fs::DirBuilder::new()
//         .recursive(true)
//         .create(&lair_root)
//         .await?;

//     let config = LairServerConfigInner::new(lair_root, passphrase).await?;

//     let mut config_f = tokio::fs::OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open(config_path)
//         .await?;

//     config_f.write_all(config.to_string().as_bytes()).await?;
//     config_f.shutdown().await?;
//     drop(config_f);

//     Ok(Arc::new(config))
// }
