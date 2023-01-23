// TODO: ask Fabian if this should be kept in release
/* #![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
 */
mod content_manager;
mod database;
mod directory;
mod file_utils;
mod foc_error;
mod searcher;
mod web_extension;

use crate::{
    content_manager::get_all_tags,
    models::smart_folder::{
        add_folder, delete_folder, get_all_folders, validate_folder_name, validate_folder_path,
    },
    preferences::{get_preferences, update_preferences},
    searcher::search_content,
};
use crate::{content_manager::update_content, searcher::get_metadata_by_hashes};
use crate::{
    content_manager::{add_files, delete_content},
    directory::open_in_explorer,
};
use crate::{database::init_database, file_utils::get_file_size};

#[macro_use]
extern crate diesel;

use content_manager::check_file_integrity;
use directory::create_base_dirs;
use foc_error::Result;
use searcher::init_reader;
use web_extension::{check_extension_requests, register};

pub mod models;
pub mod preferences;
pub mod schema;

pub fn main() -> Result<()> {
    check_extension_requests("com.envoid.focular")?;
    create_base_dirs()?;

    tauri::Builder::default()
        .setup(|_app| {
            let app_handle = _app.handle();
            register(app_handle)?;
            println!("Registered web extension");
            // TODO: handle database and reader errors
            match tauri::async_runtime::block_on(init_database()) {
                Ok(_test) => {}
                Err(error) => panic!("Problem initializing database: {:?}", error),
            };
            println!("Initialized database");
            match tauri::async_runtime::block_on(init_reader()) {
                Ok(_test) => {}
                Err(error) => panic!("Problem initializing reader: {:?}", error),
            };

            println!("Initialized reader");
            match tauri::async_runtime::block_on(check_file_integrity()) {
                Ok(_test) => {}
                Err(error) => panic!("Problem checking file integrity: {:?}", error),
            };
            println!("Checked file integrity");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_files,
            search_content,
            get_metadata_by_hashes,
            delete_content,
            open_in_explorer,
            get_file_size,
            update_content,
            get_all_tags,
            get_preferences,
            update_preferences,
            validate_folder_name,
            validate_folder_path,
            get_all_folders,
            add_folder,
            delete_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
