import { listen } from '@tauri-apps/api/event';
import { addModFiles, saveSettings, updateMods } from './ipc';
import { writable } from 'svelte/store';

const _settings = writable({ game_data_path: "" } as Settings);
const _mods = writable(modsInit());

export async function subscribeToTauriEvents() {
  await listen<Settings>("Event", (event) => {
    alert(`${JSON.stringify(event)}`);
  });
  let unsubscribeSettingsUpdates = await listen<Settings>("settings-event", (event) => {
    console.log(`settings-event ${event}`);
    _settings.set(event.payload);
  });

  let unsubscribeModsUpdates = await listen<Mod[]>("mods-event", (event) => {
    console.log(`mods-event ${event}`);
    _mods.set(event.payload);
  });

  return () => {
    unsubscribeSettingsUpdates();
    unsubscribeModsUpdates();
  };
};


export const settings = {
  subscribe: _settings.subscribe,
  saveSettings
}


export const mods = {
  subscribe: _mods.subscribe,
  addModFiles,
  updateMods
}

interface Settings {
  game_data_path: string,
}

interface Mod {
  author: String,
  name: String,
  uuid: String,
  folder: String,
  md5: String,
  description: String,
  version: String | null,
  created: String,
  dependencies: String[],
  group: String,
  is_enabled: boolean,
  order: number,
}

function modsInit(): Mod[] {
  return [
    {
      "author": "Eldric Stormblade",
      "name": "Sorcerer's Grimoire",
      "folder": "ArcaneArtifacts",
      "version": null,
      "description": "Unleash the power of ancient spells with this mystical grimoire.",
      "uuid": "c9e5a97a-62ed-4f62-bf4a-8c94dbd6c28d",
      "created": "2023-10-15T08:30:45.1234567-06:00",
      "dependencies": [],
      "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
      "is_enabled": true,
      "order": 0,
      "md5": "a1b2c3d4e5f6g7h8i9j0"
    },
    {
      "author": "Liliana Shadowdancer",
      "name": "Shadow's Veil",
      "folder": "StealthEssentials",
      "version": null,
      "description": "Become one with the shadows and vanish from sight with this stealth mod.",
      "uuid": "d8f7e6b5-a4c3-4f2e-9d1c-0b9a8f7e6d5c",
      "created": "2023-11-20T14:45:30.9876543-06:00",
      "dependencies": [],
      "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
      "is_enabled": true,
      "order": 0,
      "md5": "b2a3c4d5e6f7g8h9i0j1k2l3"
    },
    {
      "author": "Dwarven Smithy",
      "name": "Forgemaster's Arsenal",
      "folder": "CraftingExpansions",
      "version": null,
      "description": "Craft legendary weapons and armor with the secrets of the dwarven forgemasters.",
      "uuid": "f1e2d3c4-b5a6-4e7d-8c9b-0a1b2c3d4e5f",
      "created": "2023-12-25T18:15:10.3456789-06:00",
      "dependencies": [],
      "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
      "is_enabled": true,
      "order": 0,
      "md5": "c3d4e5f6g7h8i9j0k1l2m3"
    },
    {
      "author": "Elowen Moonshadow",
      "name": "Forest Guardian's Blessing",
      "folder": "NatureEnhancements",
      "version": null,
      "description": "Harness the power of the ancient forest with this mystical blessing.",
      "uuid": "e4f5d6c7-b8a9-4d0e-1c2b-3a4b5c6d7e8f",
      "created": "2023-08-05T10:20:35.5678901-06:00",
      "dependencies": [],
      "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
      "is_enabled": false,
      "order": 0,
      "md5": "d4e5f6g7h8i9j0k1l2m3n4"
    },
    {
      "author": "Ragnar Thunderbeard",
      "name": "Dwarven Stronghold",
      "folder": "EpicFortresses",
      "version": null,
      "description": "Build a mighty dwarven stronghold and defend it from invaders.",
      "uuid": "1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d",
      "created": "2023-07-10T12:55:20.2345678-06:00",
      "dependencies": [],
      "group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df",
      "is_enabled": false,
      "order": 0,
      "md5": "e5f6g7h8i9j0k1l2m3n4o5"
    }
  ];
}
