pub mod schema;
pub mod models;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;

pub struct Database(Mutex<SqliteConnection>);

impl Database{
    pub fn new() -> Self{
        //TODO
        //Add generation of new database if it doesnt exist
        Database(Mutex::new(Database::connect()))
    }

    fn connect() -> SqliteConnection{
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<SqliteConnection>{
        self.0.lock().unwrap()
    }
}

impl Default for Database{
    fn default() -> Self {
        Database::new()
    }
}