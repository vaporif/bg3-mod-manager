/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function filesDropped(filePaths: string[]) {
    return invoke()<null>("files_dropped", { filePaths })
}

export function saveSettings(settings: Settings) {
    return invoke()<null>("save_settings", { settings })
}

export function getDefaultGameDataPath() {
    return invoke()<string>("get_default_game_data_path")
}

export type Settings = { game_data_path: string | null }
