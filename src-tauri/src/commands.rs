use crate::database::{models::*, Database};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command]
pub fn insert(db: State<Database>, args: Tables) -> Result<InsertReturn, tauri::InvokeError> {
    args.insert(&mut db.get_connection())
        .map_err(|_| tauri::InvokeError::from(""))
}

#[derive(Deserialize)]
pub enum Tables {
    Store(NewStore),
    Promoter(NewPromoter),
    Model(NewModel),
    Promotion(NewPromotion),
    Purchase(NewPurchase),
}

impl Tables {
    fn insert(
        self,
        connection: &mut SqliteConnection,
    ) -> Result<InsertReturn, diesel::result::Error> {
        match self {
            Tables::Store(data) => data.insert(connection).map(InsertReturn::Store),
            Tables::Promoter(data) => data.insert(connection).map(InsertReturn::Promoter),
            Tables::Model(data) => data.insert(connection).map(InsertReturn::Model),
            Tables::Promotion(data) => data.insert(connection).map(InsertReturn::Promotion),
            Tables::Purchase(data) => data.insert(connection).map(InsertReturn::Purchase),
        }
    }
}

#[derive(Serialize)]
pub enum InsertReturn {
    Store(Store),
    Promoter(Promoter),
    Model(Model),
    Promotion(Promotion),
    Purchase(Purchase),
    Payment(Payment),
}
