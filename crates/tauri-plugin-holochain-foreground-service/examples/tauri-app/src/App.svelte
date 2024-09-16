<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import happUrl from "./forum.happ?url";

  let adminPort;
  let appId = "forum";
  let networkSeed = "p2p-shipyard-dev-2024-09-16";

  const getAdminPort = async () => {
    adminPort = (await invoke('plugin:holochain-foreground-service|get_admin_port')).port;
  };
  const launch = () => invoke('plugin:holochain-foreground-service|launch');
  const shutdown = () => invoke('plugin:holochain-foreground-service|shutdown');
  const installApp = async () => {
    const appBundleBytes = new Uint8Array(await (await fetch(happUrl)).arrayBuffer())

    return invoke('plugin:holochain-foreground-service|install_app', {
      appId,
      appBundleBytes,
      membraneProofs: {},
      agent: null,
      networkSeed,
    })
  };

  let interval = setInterval(async () => {
    if(!adminPort) {
      await getAdminPort();
    } else {
      clearInterval(interval);
    }
  }, 500);
</script>

<main class="container">
  <h1 style="line-height: 2.5rem;">tauri-plugin-holochain-foreground-service demo</h1>

  <div class="my-4">
    <h2>Holochain Conductor Service</h2>
    <div>
      <button on:click={launch}>Launch</button>
      <button on:click={shutdown}>Shutdown</button>
    </div>
  </div>

  <div class="my-4">
   <h2>Admin Port</h2>
    <div style="margin-top: 10px;">
      <pre>{adminPort}</pre>
    </div>
  </div>

  <div class="my-4">
    <h2>Install Forum App</h2>
    <div>
      <b>App Id:</b> <input bind:value={appId} />
    </div>
    <div>
      <b>Network Seed:</b> <input bind:value={networkSeed} />
    </div>
    <div>
      <button on:click={installApp}>Install App</button>
    </div>
   </div>
</main>

<style>
  .my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
</style>