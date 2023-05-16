use std::ops::Deref;

use diesel::{Insertable, SqliteConnection};
use serde::Deserialize;
use tauri::State;
use crate::database::{Database, models::*};

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

#[tauri::command]
pub fn insert(
    db: State<Database>,
    args: Tables
) -> Result<Store, tauri::InvokeError>{
    args.insert(&mut db.get_connection())
        .map_err(|_| tauri::InvokeError::from(""))
}

#[derive(Deserialize)]
pub enum Tables{
    Store(NewStore),
    Promoter(NewPromoter),
    Model(NewModel),
    Promotion(NewPromotion),
    Purchase(NewPurchase),
    Payment(NewPayment),
}

impl Tables{
    fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        match self{
            Tables::Store(data) => data.insert(connection),
            Tables::Promoter(data) => data.insert(connection),
            Tables::Model(data) => data.insert(connection),
            Tables::Promotion(data) => data.insert(connection),
            Tables::Purchase(data) => data.insert(connection),
            Tables::Payment(data) => data.insert(connection),
        }
    }
}