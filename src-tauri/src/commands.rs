use serde::Deserialize;
use tauri::State;
use crate::database::{Database, models::NewStore};

#[derive(Deserialize, Debug)]
pub struct FormData{
    loja: String,
    promotor: String, 
    modelo: String,
    valor: i32
}

#[tauri::command]
pub fn submit_form(
    args: FormData
){
    println!("{args:?}");
}

#[derive(Deserialize)]
pub struct Store{
    pub name: String
}

#[tauri::command]
pub fn new_store(
    db: State<Database>,
    args: Store
) -> Result<usize, tauri::InvokeError>{
    NewStore::new(args.name)
        .insert(&mut db.get_connection())
        .map_err(|_| tauri::InvokeError::from(""))
}