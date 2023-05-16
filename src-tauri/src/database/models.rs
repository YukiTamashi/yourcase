use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::database::schema::*;

#[derive(Queryable, Identifiable, PartialEq, Debug, Deserialize, Serialize)]
#[diesel(table_name = store)]
pub struct Store {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = store)]
pub struct NewStore {
    pub name: String,
}

impl NewStore{
    pub fn new(name: String) -> Self{
        Self{name}
    }
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error> {
        diesel::insert_into(store::table)
            .values(&self)
            .get_result(connection)
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = promoter)]
pub struct Promoter {
    pub id: i32,
    pub store_id: i32,
    pub name: String,
    pub active: bool,
    pub bank_id: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = promoter)]
pub struct NewPromoter{
    pub store_id: i32,
    pub name: String,
    pub bank_id: Option<String>,
}

impl NewPromoter{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        todo!()
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = model)]
pub struct Model {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = model)]
pub struct NewModel{
    pub name: String,
}

impl NewModel{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        todo!()
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = promotion)]
pub struct Promotion {
    pub id: i32,
    pub promoter_id: i32,
    pub model_id: i32,
    pub date: i64,
    pub paid: bool,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = promotion)]
pub struct NewPromotion{
    pub promoter_id: i32,
    pub model_id: i32,
}

impl NewPromotion{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        todo!()
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = purchase)]
pub struct Purchase {
    pub id: i32,
    pub date: i32,
    pub description: String,
    pub value: i32,
    pub debt: i32,
    pub promoter_id: i32,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = purchase)]
pub struct NewPurchase{
    pub id: i32,
    pub promoter_id: i32,
    pub description: String,
}

impl NewPurchase{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        todo!()
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = payment)]
pub struct Payment {
    pub id: i32,
    pub promoter_id: i32,
    pub value: i32,
    pub date: i32,
    pub net_received: i32,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = payment)]
pub struct NewPayment{
    pub promoter_id: i32,
    pub value: i32,
}

impl NewPayment{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error>{
        todo!()
    }
}