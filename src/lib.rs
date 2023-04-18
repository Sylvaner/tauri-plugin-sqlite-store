use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, State, AppHandle
};
use std::{collections::HashMap, sync::Mutex, fs, path::Path};
use rusqlite::Connection;
use serde_json::{Value as JsonValue};
mod sqlite;
mod error;
mod models;

use sqlite::{connect, execute as sqlite_execute, select as sqlite_select, batch as sqlite_batch, close as sqlite_close};
use error::{Error, Result};
use models::{OpenOptions};

/// Default name of the store in appdata directory
const STORE_FILENAME: &str = "store.sqlite";

#[derive(Default)]
/// Hashmap containing the different connections
struct DbInstances (
    Mutex<HashMap<String, Connection>>
);

/// Open database
/// 
/// # Arguments
/// 
/// `state` - State of the plugin that contains connections
/// `db_path` - Path of the db
/// `options` - Open options
/// 
fn open_db(state: State<'_, DbInstances>, db_path: String, options: OpenOptions) -> Result<bool> {
    match connect(&db_path) {
        Ok(mut conn) => {
            if let Some(disable_foreign_keys) = options.disable_foreign_keys {
                if disable_foreign_keys == true {
                    _ = sqlite_execute(&mut conn, "PRAGMA foreign_keys = 0", vec!());
                }
            }
            state.0.lock().unwrap().insert(db_path, conn);
            Ok(true)
        }    
        Err(e) => Err(Error::Rusqlite(e))
    }
}

#[tauri::command]
/// Load local database from appdata directory
/// File is named store.sqlite
/// 
/// # Arguments
/// 
/// * `app` - Application context
/// * `state` - State of the plugin that contains connections
/// * `options` - Open options
///
async fn load<R: Runtime>(app: AppHandle<R>, state: State<'_, DbInstances>, options: OpenOptions) -> Result<String> {
    let app_dir = app.path_resolver().app_data_dir().expect("Failed to resolve app_dir");
    let db_path = app_dir.join(STORE_FILENAME).as_path().display().to_string();
    if !Path::new(&app_dir).exists() {
        fs::create_dir_all(app_dir)?;
    }
    match open_db(state, db_path.clone(), options) {
        Ok(_) => Ok(db_path),
        Err(e) => Err(e)
    }
}

#[tauri::command]
/// Open database on computer
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// * `options` - Open options
///
async fn open(state: State<'_, DbInstances>, db_path: String, options: OpenOptions) -> Result<bool> {
    open_db(state, db_path, options)
}

#[tauri::command]
/// Set pragma value
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// * `key` - Pragma key
/// * `value` - Value to set
/// 
async fn set_pragma(state: State<'_, DbInstances>, db_path: String, key: String, value: JsonValue) -> Result<bool> {
    let mut mutex_map = state.0.lock().unwrap();
    let mut conn = mutex_map.get_mut(&db_path).ok_or(Error::NotConnected(db_path))?;
    match sqlite_execute(&mut conn, format!("PRAGMA {} = {}", key, value).as_str(), vec!()) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::Rusqlite(e))
    }
}

#[tauri::command]
/// Select query in the database
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// * `query` - SQL query
/// * `params` - SQL Params in the query
/// 
async fn select(state: State<'_, DbInstances>, db_path: String, query: String, params: Vec<JsonValue>) -> Result<Vec<HashMap<String, JsonValue>>> {
    let mut mutex_map = state.0.lock().unwrap();
    let conn = mutex_map.get_mut(&db_path).ok_or(Error::NotConnected(db_path))?;
    match sqlite_select(&conn, query.as_str(), params) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::Rusqlite(e))
    }
}

#[tauri::command]
/// Execute query in the database
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// * `query` - SQL query
/// * `params` - SQL Params in the query
/// 
async fn execute(state: State<'_, DbInstances>, db_path: String, query: String, params: Vec<JsonValue>) -> Result<bool> {
    let mut mutex_map = state.0.lock().unwrap();
    let mut conn = mutex_map.get_mut(&db_path).ok_or(Error::NotConnected(db_path))?;
    match sqlite_execute(&mut conn, query.as_str(), params) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::Rusqlite(e))
    }
}

#[tauri::command]
/// Execute list of queries
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// * `queries` - Tuple with SQL query and associated params
/// 
async fn batch(state: State<'_, DbInstances>, db_path: String, queries: Vec<(&str, Vec<JsonValue>)>) -> Result<bool> {
    let mut mutex_map = state.0.lock().unwrap();
    let mut conn = mutex_map.get_mut(&db_path).ok_or(Error::NotConnected(db_path))?;
    match sqlite_batch(&mut conn, queries) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::Rusqlite(e))
    }
}

#[tauri::command]
/// Close the database
/// 
/// # Arguments
/// 
/// * `state` - State of the plugin that contains connections
/// * `db_path` - Path of the database file
/// 
async fn close(state: State<'_, DbInstances>, db_path: String) -> Result<bool> {
    let mut mutex_map = state.0.lock().unwrap();
    let conn = mutex_map.remove(&db_path).ok_or(Error::NotConnected(db_path))?;
    println!("{:?}", sqlite_close(conn));
    Ok(true)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("sqlite-store")
    .invoke_handler(tauri::generate_handler![open, select, execute, set_pragma, batch, load, close])
    .setup(|app| {
        app.manage(DbInstances::default());
        Ok(())
    })
    .build()
}
