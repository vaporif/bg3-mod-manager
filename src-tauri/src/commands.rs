use std::path::PathBuf;

use specta::Type;
use tauri::{Builder, State, Wry};

use crate::mods::{Mod, ModInfo, ZippedMod, InstalledMod};
use crate::settings::Settings;
use crate::state::Store;
use crate::{error, prelude::*};

#[derive(serde::Serialize, Type)]
struct ModFileAddError {
    pub path: PathBuf,
    pub error: String,
}

#[derive(serde::Serialize, Type)]
struct AddModFileResult {
    valid_files: Vec<InstalledMod>,
    invalid_files: Vec<ModFileAddError>,
}

pub fn register_command_handlers(builder: Builder<Wry>) -> Builder<Wry> {
    // Specta generating typed binding interfaces
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![
            add_mod_files,
            update_mods,
            save_settings,
            get_default_game_data_path,
        ],
        "../src/lib/ipc.ts",
    )
    .expect("unable to generate ts typings for commands");

    // tauri_specta::ts::export(
    //     specta::ts::export::<Event>(&ExportConfiguration::default()).unwrap(),
    //     "../src/lib/events.ts")
    // .expect("unable to generate ts typings for events");

    builder.invoke_handler(tauri::generate_handler![
        add_mod_files,
        update_mods,
        save_settings,
        get_default_game_data_path
    ])
}

#[tauri::command]
#[specta::specta]
fn add_mod_files(file_paths: Vec<PathBuf>) -> AddModFileResult {
    let mut result = AddModFileResult {
        valid_files: Vec::new(),
        invalid_files: Vec::new(),
    };

    for path in file_paths {
        let path_err = path.clone();
        match ZippedMod::from_file(path) {
            Ok(zip_archive) => result.valid_files.push(zip_archive.into()),
            Err(mod_add_error) => {
                error!("zip error{}", &mod_add_error);
                result.invalid_files.push(ModFileAddError {
                    path: path_err,
                    error: mod_add_error.to_string(),
                });
            }
        }
    }

    result
}

#[tauri::command]
#[specta::specta]
fn update_mods(mods: Vec<Mod>) {
    todo!()
}

#[tauri::command]
#[specta::specta]
async fn save_settings(settings: Settings, state: State<'_, Store>) -> Result<()> {
    state.update_settings(settings).await?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
fn get_default_game_data_path() -> String {
    let res = &crate::settings::DEFAULT_GAME_PATH;

    let res = res.to_string_lossy().into_owned();

    info!("{}", res);
    res
}
