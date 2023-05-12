use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::database::schema::*;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = store)]
pub struct Store {
    pub id: i32,
    pub name: String,
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

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = model)]
pub struct Model {
    pub id: i32,
    pub name: String,
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

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = purchase)]
pub struct Purchase {
    pub id: i32,
    pub promoter_id: i32,
    pub item: String,
    pub date: i64,
    pub debt_remaining: f64,
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = payment)]
pub struct Payment {
    pub id: i32,
    pub promoter_id: i32,
    pub value: f64,
    pub date: i64,
    pub net_received: f64,
}