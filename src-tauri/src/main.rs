// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct FormData{
    loja: String,
    promotor: String, 
    modelo: String,
    valor: i32
}

#[tauri::command]
fn submit_form(
){
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![submit_form])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
