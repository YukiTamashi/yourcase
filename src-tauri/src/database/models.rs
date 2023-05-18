use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use super::schema::*;


#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct Store {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = stores)]
pub struct NewStore {
    pub name: String,
}

impl NewStore{
    pub fn new(name: String) -> Self{
        Self{name}
    }
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Store, diesel::result::Error> {
        diesel::insert_into(stores::dsl::stores)
            .values(&self)
            .get_result(connection)
    }
}

#[derive(Serialize, Queryable, Selectable, Identifiable, PartialEq, Debug)]
pub struct Promoter {
    pub id: i32,
    pub name: String,
    pub bank_id: Option<String>,
    pub store_id: i32,
    pub active: i32,
    pub debt: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = promoters)]
pub struct NewPromoter{
    pub id: i32,
    pub name: String,
    pub store_id: i32,
}

impl NewPromoter{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Promoter, diesel::result::Error>{
        diesel::insert_into(promoters::dsl::promoters)
            .values(&self)
            .get_result(connection)
    }
}

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct Model {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = models)]
pub struct NewModel{
    pub name: String,
}

impl NewModel{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Model, diesel::result::Error>{
        diesel::insert_into(models::dsl::models)
            .values(&self)
            .get_result(connection)
    }
}

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct Promotion {
    pub id: i32,
    pub date: i32,
    pub value: i32,
    pub model_id: i32,
    pub promoter_id: i32,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = promotions)]
pub struct NewPromotion{
    pub promoter_id: i32,
    pub model_id: i32,
    pub value: i32
}

impl NewPromotion{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Promotion, diesel::result::Error>{
        todo!()
    }
}

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct Purchase {
    pub id: i32,
    pub date: i32,
    pub description: String,
    pub value: i32,
    pub debt: i32,
    pub promoter_id: i32,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = purchases)]
pub struct NewPurchase{
    pub id: i32,
    pub promoter_id: i32,
    pub description: String,
}

impl NewPurchase{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Purchase, diesel::result::Error>{
        todo!()
    }
}

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct Payment {
    pub id: i32,
    pub promoter_id: i32,
    pub value: i32,
    pub date: i32,
    pub net_received: i32,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = payments)]
pub struct NewPayment{
    pub promoter_id: i32,
    pub value: i32,
}

impl NewPayment{
    pub fn insert(self, connection: &mut SqliteConnection) -> Result<Payment, diesel::result::Error>{
        todo!()
    }
}