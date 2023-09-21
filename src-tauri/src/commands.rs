use crate::database::{models::*, Database};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command]
pub fn insert(db: State<Database>, args: Insertable) -> Result<Select, tauri::InvokeError> {
    args.insert(&mut db.get_connection())
        .map_err(|_| tauri::InvokeError::from(""))
}

#[derive(Deserialize)]
pub enum Insertable {
    Store(NewStore),
    Promoter(NewPromoter),
    Model(NewModel),
    Promotion(NewPromotion),
    Purchase(NewPurchase),
}

impl Insertable {
    fn insert(self, connection: &mut SqliteConnection) -> Result<Select, diesel::result::Error> {
        match self {
            Insertable::Store(data) => data.insert(connection).map(Select::Store),
            Insertable::Promoter(data) => data.insert(connection).map(Select::Promoter),
            Insertable::Model(data) => data.insert(connection).map(Select::Model),
            Insertable::Promotion(data) => data.insert(connection).map(Select::Promotion),
            Insertable::Purchase(data) => data.insert(connection).map(Select::Purchase),
        }
    }
}

#[derive(Serialize)]
pub enum Select {
    Store(Store),
    Promoter(Promoter),
    Model(Model),
    Promotion(Promotion),
    Purchase(Purchase),
    Payment(Payment),
}
