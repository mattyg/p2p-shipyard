/// Helper functions to wrap tauri plugin commands
import { invoke } from '@tauri-apps/api/core';
export async function installApp(request) {
    return await invoke('plugin:holochain-service-consumer|install_app', request);
}
export async function isAppInstalled(appId) {
    return await invoke('plugin:holochain-service-consumer|is_app_installed', { appId }).then((r) => (r.installed));
}
export async function appWebsocketAuth(appId) {
    return await invoke('plugin:holochain-service-consumer|app_websocket_auth', { appId });
}
