/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export async function requestInstallApp(request: {
  appId: string,
  appBundleBytes: Uint8Array,
  membraneProofs: Map<String, Uint8Array>,
  agent?: Uint8Array,
  networkSeed: String,
}): Promise<{appId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{appId: string, port: number, token: Uint8Array}>('plugin:holochain-foreground-service-consumer|request_install_app', request).then((r) => (r.appWebsocketAuth ? r.appWebsocketAuth : null));
}
