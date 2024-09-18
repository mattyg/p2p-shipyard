/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export async function launch(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:holochain-foreground-service|launch', {
    payload: {},
  }).then((r) => (r.value ? r.value : null));
}

export async function stop(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:holochain-foreground-service|stop', {
    payload: {},
  }).then((r) => (r.value ? r.value : null));
}

export async function getAdminPort(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:holochain-foreground-service|get_admin_port', {
    payload: {},
  }).then((r) => (r.value ? r.value : null));
}
