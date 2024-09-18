<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Labelled from './Labelled.svelte';
  import happUrl from "./forum.happ?url";
  import { AppWebsocket } from "@holochain/client";
  import { decode } from "@msgpack/msgpack";

  let adminPort;
  let appId = "forum";
  let networkSeed = "p2p-shipyard-dev-2024-09-16";
  let installedApps = [];
  let selectedAppId;
  let selectedAppWebsocketAuth;
  let newPost = {title: "", content: ""};
  let allPosts = [];
  let appWs: AppWebsocket | null = null;

  const loadHolochainClient = async () => {
    appWs = await AppWebsocket.connect();
  }
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
  const appWebsocketAuth = async () => {
    selectedAppWebsocketAuth = (await invoke('plugin:holochain-foreground-service|app_websocket_auth', { 
      appId: selectedAppId 
    })).appWebsocketAuth;
  }
  const callZomeCreatePost = async () => {
    if(!appWs) throw Error("No AppWebsocket connected");

    appWs.callZome({
      role_name: "forum",
      zome_name: "posts",
      fn_name: "create_post",
      payload: newPost,
    });
  }
  const callZomeGetAllPosts = async () => {
    if(!appWs) throw Error("No AppWebsocket connected");

   const links = await appWs.callZome({
      cap_secret: null,
      role_name: 'forum',
      zome_name: 'posts',
      fn_name: 'get_all_posts',
      payload: null,
    });
    const hashes = links.map(l => l.target);

    allPosts = [];

    await Promise.all(hashes.map(async (hash) => {
      const record = await appWs.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'posts',
        fn_name: 'get_latest_post',
        payload: hash,
      });
      if (record) {
        const post = decode((record.entry as any).Present.entry);
        allPosts = [...allPosts, post];
      }
    }));
  }

  let interval = setInterval(async () => {
    if(!adminPort) {
      await getAdminPort();
    } else {
      clearInterval(interval);
    }
  }, 500);

  $: adminPort, listInstalledApps();
  $: selectedAppWebsocketAuth, loadHolochainClient();
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
            <li>{app.installedAppId}</li>
          {/each}
      </ul>
      <Labelled label="Selected App">
        <select bind:value={selectedAppId}>
          {#each installedApps as app}
            <option value={app.installedAppId}>{app.installedAppId}</option>
          {/each}
        </select>
      </Labelled>
   </div>

   <div class="my-4 flex-center">
    <h2>App Websocket Auth</h2>
      <div>
        <button on:click={appWebsocketAuth} disabled={selectedAppId === null}>Get Selected App Websocket Auth</button>
      </div>
      {#if selectedAppWebsocketAuth}
        <Labelled label="App Id">
          {selectedAppWebsocketAuth.appId}
        </Labelled>
        <Labelled label="Port">
          {selectedAppWebsocketAuth.port}
        </Labelled>
        <Labelled label="Token">
          <div style="width: 200px; word-break: break-all;">{selectedAppWebsocketAuth.token}</div>
        </Labelled>
      {/if}
   </div>

   <div class="my-4 flex-center">
    <h2>Zome Call</h2>
    <p>Call zome fn "create_post"</p>
    <Labelled label="title">
      <input bind:value={newPost.title} />
    </Labelled>
    <Labelled label="content">
      <input bind:value={newPost.content} />
    </Labelled>
    <div>
      <button on:click={callZomeCreatePost} disabled={newPost.title?.length === 0 || newPost.content?.length === 0}>Create Post</button>
    </div>
   </div>

   <div class="my-4 flex-center">
    <h2>Zome Call</h2>
    <p>Call zome fn "get_all_posts"</p>
    <div>
      <button on:click={callZomeGetAllPosts}>Get All Posts</button>
    </div>
    <ul>
      {#each allPosts as post}
       <li>
        <Labelled label="Title">{post.title}</Labelled>
        <Labelled label="Content">{post.content}</Labelled>
       </li>
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