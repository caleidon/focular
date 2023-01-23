use serde::{Deserialize, Serialize};
use std::fs::File;
use ts_rs::TS;

use crate::{
    directory::{get_dir_path, Dir},
    foc_error::Result,
};

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Preferences {
    pub recent_searches: Vec<String>,
    pub show_file_extensions: bool,
}

pub fn create_if_not_exists() -> Result<()> {
    let preferences_path = get_dir_path(Dir::Data)?.join("preferences.json");
    if preferences_path.exists() {
        return Ok(());
    }

    let preferences = Preferences {
        recent_searches: Vec::new(),
        show_file_extensions: false,
    };

    let mut file = File::create(preferences_path)?;
    serde_json::to_writer_pretty(&mut file, &preferences)?;

    Ok(())
}

#[tauri::command]
pub fn get_preferences() -> Result<Preferences> {
    create_if_not_exists()?;

    let preferences_path = get_dir_path(Dir::Data)?.join("preferences.json");
    let mut file = File::open(preferences_path)?;
    let preferences: Preferences = serde_json::from_reader(&mut file)?;
    Ok(preferences)
}

#[tauri::command]
pub fn update_preferences(preferences: Preferences) -> Result<()> {
    let preferences_path = get_dir_path(Dir::Data)?.join("preferences.json");
    let mut file = File::create(preferences_path)?;
    serde_json::to_writer_pretty(&mut file, &preferences)?;
    Ok(())
}
