<script lang="ts">
  import Labelled from './Labelled.svelte';
  import { invoke } from "@tauri-apps/api/core";
  import happUrl from "./forum.happ?url";

  let adminPort;
  let appId = "forum";
  let networkSeed = "p2p-shipyard-dev-2024-09-16";
  let installedApps = [];

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
  const listInstalledApps = async () => {
    installedApps = (await invoke('plugin:holochain-foreground-service|list_installed_apps')).installedApps;
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

  <div class="my-4 flex-center">
    <h2>Admin Port</h2>
    <button on:click={getAdminPort}>Get Admin Port</button>
    <pre>{adminPort}</pre>
  </div>

  <div class="my-4 flex-center">
    <h2>Install Forum App</h2>
    <Labelled label="App Id">
      <input bind:value={appId} />
    </Labelled>
    <Labelled label="Network Seed">
      <input bind:value={networkSeed} />
    </Labelled>
    <div>
      <button on:click={installApp}>Install App</button>
    </div>
  </div>
  
  <div class="my-4 flex-center">
    <h2>Installed Apps</h2>
      <div>
        <button on:click={listInstalledApps}>List Installed Apps</button>
      </div>
      <ul style="margin-top: 10px;">
          {#each installedApps as app}
            <li>{app}</li>
          {/each}
      </ul>
   </div>
 
</main>

<style>
  .my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
  .flex-center {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>