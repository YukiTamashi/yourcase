use diesel::{Insertable, SqliteConnection};
use serde::Deserialize;
use tauri::State;
use crate::database::{Database, models::{NewStore, Store}};

#[derive(Deserialize, Debug)]
pub struct FormData{
    loja: String,
    promotor: String, 
    modelo: String,
    valor: i32
}

pub trait DatabaseType<'a, U>: std::fmt::Debug + Deserialize<'a>{
    //fn insert(self, connection: &mut SqliteConnection) -> Result<U, diesel::result::Error>;
}

#[tauri::command]
pub fn submit_form(
    args: FormData
){
    println!("{args:?}");
}

#[tauri::command]
pub fn new_store(
    db: State<Database>,
    args: NewStore
) -> Result<Store, tauri::InvokeError>{
    args.insert(&mut db.get_connection())
        .map_err(|_| tauri::InvokeError::from(""))
}

#[tauri::command]
pub fn test/*<'a, U, T: DatabaseType<'a, U>>*/(
    db: State<Database>,
    /*args: T,*/
)/* -> Result<U, tauri::InvokeError>*/{
    println!("ok");
    //args.insert(&mut db.get_connection()).map_err(|_| tauri::InvokeError::from(""))
}

#[derive(Deserialize, Debug)]
struct TestA{
    name: String
}

impl<'a> DatabaseType<'a, TestC> for TestA{
    
}

pub struct TestC{}

impl<'a> DatabaseType<'a, TestC> for (){}