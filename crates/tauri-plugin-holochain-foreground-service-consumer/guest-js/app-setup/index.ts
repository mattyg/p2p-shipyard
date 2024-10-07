import { installApp, appWebsocketAuth, isAppInstalled } from "tauri-plugin-holochain-foreground-service-consumer-api";

async function invokeSetupApp(appId: string, appBundleBytes: Uint8Array, membraneProofs: Map<string, Uint8Array>, agent: Uint8Array, networkSeed: string) {
  const isInstalled = await isAppInstalled(appId);

  if(!isInstalled) {
    await installApp({
      appId,
      appBundleBytes,
      membraneProofs,
      agent,
      networkSeed,
    })
  }

  await appWebsocketAuth(appId);
}

(window as any).invokeSetupApp = invokeSetupApp;