<script>
  import { invoke } from "@tauri-apps/api/core";

  let adminPort;

  const getAdminPort = async () => {
    adminPort = (await invoke('plugin:holochain-foreground-service|get_admin_port')).port;
  };
  const start = () => invoke('plugin:holochain-foreground-service|start');
  const stop = () =>invoke('plugin:holochain-foreground-service|stop');

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
    <h2>Service</h2>
    <div>
      <button on:click={start}>Start service</button>
      <button on:click={stop}>Stop service</button>
    </div>
  </div>

  <div class="my-4">
   <h2>Admin Port</h2>
    <div style="margin-top: 10px;">
      <pre>{JSON.stringify(adminPort)}</pre>
    </div>
  </div>
</main>

<style>
  .my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
</style>