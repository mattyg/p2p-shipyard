/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export async function launch(): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|launch');
}

export async function shutdown(): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|shutdown');
}

export async function getAdminPort(): Promise<number | null> {
  return await invoke<{port?: number}>('plugin:holochain-foreground-service|get_admin_port').then((r) => (r.port ? r.port : null));
}

export async function installApp(request: {
  appId: string,
  appBundleBytes: Uint8Array,
  membraneProofs: Map<String, Uint8Array>,
  agent?: Uint8Array,
  networkSeed: String,
}): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|install_app', request);
}

export async function uninstallApp(appId: string): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|uninstall_app', { appId });
}

export async function enableApp(appId: string): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|enable_app', { appId });
}

export async function disableApp(appId: string): Promise<string | null> {
  return await invoke('plugin:holochain-foreground-service|disable_app', { appId });
}

export async function listInstalledApps(): Promise<{installedAppId: string}[]> {
  return await invoke<{installedApps: {installedAppId: string}[]}>('plugin:holochain-foreground-service|list_installed_apps').then((r) => (r.installedApps ? r.installedApps : []));
}

export async function appWebsocketAuth(appId: string): Promise<{appId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{appWebsocketAuth: {appId: string, port: number, token: Uint8Array}}>('plugin:holochain-foreground-service|app_websocket_auth', { appId }).then((r) => (r.appWebsocketAuth ? r.appWebsocketAuth : null));
}
