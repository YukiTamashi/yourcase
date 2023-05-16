// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use yourcase::{commands::*, database::Database};

fn main() {
    let database = Database::default();
    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            submit_form, 
            new_store,
            test,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
