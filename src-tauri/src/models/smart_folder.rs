use std::path::{Path, PathBuf};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    database::DATABASE_INSTANCE,
    foc_error::{FocError, Result},
    schema::{self, smart_folder as smart_folder_schema},
};

#[derive(
    Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Insertable, Default, Clone, Debug,
)]
#[diesel(primary_key(path))]
#[diesel(table_name = crate::schema::smart_folder)]
#[diesel(treat_none_as_null = true)]
#[derive(TS)]
#[ts(export)]
pub struct SmartFolder {
    pub name: String,
    pub path: String,
    #[diesel(column_name = numberOfFiles)]
    pub number_of_files: i32,
}

#[tauri::command]
pub fn validate_folder_name(folder_name: &str) -> bool {
    use crate::schema::smart_folder::dsl::*;
    use crate::schema::smart_folder::table;
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();
    let results = table
        .filter(name.eq(folder_name))
        .load::<SmartFolder>(&mut conn)
        .unwrap();
    results.is_empty()
}

#[tauri::command]
pub fn validate_folder_path(folder_path: &str) -> bool {
    let folderPath2 = "/home/caleidon/Andrew Tate - Bundle/Andrew Tate - PHD/".to_string();
    use crate::schema::smart_folder::dsl::*;
    use crate::schema::smart_folder::table;
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    // get all folders from db
    let results = table.load::<SmartFolder>(&mut conn).unwrap();

    // we also have to check if this folder path is a child of an already tracked folder
    // if it is, we can't add it to the database
    let pathbuf = PathBuf::from(folderPath2);
    let mut current_path = pathbuf.parent();
    let latestParentPath: &Path;

    while let Some(latest_parent_path) = current_path {
        println!("latest parent path {:?}", latest_parent_path);
        current_path = latest_parent_path.parent();
    }

    /* !exists_in_db && !found_parent */
    true
}

#[tauri::command]
pub async fn get_all_folders() -> Result<Vec<SmartFolder>> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    let results = smart_folder_schema::table
        .load::<SmartFolder>(&mut conn)
        .expect("Error loading folders");

    Ok(results)
}

async fn insert_folder(new_folder: SmartFolder) -> Result<()> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    diesel::insert_into(smart_folder_schema::table)
        .values(new_folder)
        .execute(&mut conn)?;

    Ok(())
}

#[tauri::command]
pub async fn delete_folder(path: String) -> Result<()> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    diesel::delete(smart_folder_schema::table.filter(smart_folder_schema::path.eq(path)))
        .execute(&mut conn)?;

    Ok(())
}

#[tauri::command]
pub async fn add_folder(folder_name: String, folder_path: String) -> Result<()> {
    println!("Adding folder {} with path {}", folder_name, folder_path);
    if !validate_folder_name(&folder_name) || !validate_folder_path(&folder_path) {
        return Err(FocError::Folder("Invalid folder name or path".to_owned()));
    }

    let folder = SmartFolder {
        name: folder_name,
        path: folder_path,
        number_of_files: 0,
    };

    insert_folder(folder).await?;
    println!("successfully added folder");

    Ok(())
}
