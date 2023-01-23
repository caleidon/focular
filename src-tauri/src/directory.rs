use crate::foc_error::{FocError, Result};
use std::{
    ffi::OsStr,
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use tauri::api::path::{cache_dir, config_dir, data_dir};

pub enum Dir {
    Root,
    Data,
    Index,
    Thumbnails,
    Config,
}

impl Dir {
    fn to_string(&self) -> &'static str {
        match self {
            Dir::Root => "Focular",
            Dir::Data => "data",
            Dir::Index => "index",
            Dir::Thumbnails => "thumbnails",
            Dir::Config => "config",
        }
    }
}

pub fn create_base_dirs() -> Result<()> {
    // Focular directory gets created automatically
    let data_dir = get_dir_path(Dir::Data)?;
    let index_dir = get_dir_path(Dir::Index)?;
    let thumbnails_dir = get_dir_path(Dir::Thumbnails)?;
    let config_dir = get_dir_path(Dir::Config)?;
    create_dir_all(&data_dir)?;
    create_dir_all(&index_dir)?;
    create_dir_all(&thumbnails_dir)?;
    create_dir_all(&config_dir)?;
    Ok(())
}

pub fn get_dir_path(dir: Dir) -> Result<PathBuf> {
    // TODO: this is getting called waaaaay too many times

    match dir {
        Dir::Root => {
            let data_dir = match data_dir() {
                Some(data_dir) => data_dir,
                None => return Err(FocError::Fs("Could not get home directory".to_owned())),
            };
            Ok(data_dir.join(Dir::Root.to_string()))
        }
        Dir::Data => Ok(get_dir_path(Dir::Root)?.join(Dir::Data.to_string())),
        Dir::Index => Ok(get_dir_path(Dir::Root)?.join(Dir::Index.to_string())),
        Dir::Thumbnails => {
            let cache_dir = match cache_dir() {
                Some(cache_dir) => cache_dir,
                None => return Err(FocError::Fs("Could not get cache directory".to_owned())),
            };
            Ok(cache_dir
                .join(Dir::Root.to_string())
                .join(Dir::Thumbnails.to_string()))
        }
        Dir::Config => {
            let config_dir = match config_dir() {
                Some(config_dir) => config_dir,
                None => return Err(FocError::Fs("Could not get config directory".to_owned())),
            };
            Ok(config_dir
                .join(Dir::Root.to_string())
                .join(Dir::Config.to_string()))
        }
    }
}

pub fn extension_from_path(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_owned)
}

#[tauri::command]
pub fn open_in_explorer(path: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    let command = "explorer";
    #[cfg(target_os = "linux")]
    let command = "xdg-open";
    #[cfg(target_os = "macos")]
    let command = "open";

    let mut pathbuf = PathBuf::new();
    pathbuf.push(&path);

    #[cfg(target_os = "linux")]
    {
        let dir = pathbuf.parent().unwrap();
        let _ = std::process::Command::new(command).arg(&dir).spawn()?;
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let _ = std::process::Command::new(command)
        .args(["/select,", pathbuf.into_os_string().to_str().unwrap()])
        .spawn()
        .unwrap();

    Ok(())
}
