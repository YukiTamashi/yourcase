pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use directories::ProjectDirs;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

pub struct Database(Mutex<SqliteConnection>);

lazy_static! {
    pub static ref DATABASE_PATH: String = db_path();
}

fn db_path() -> String {
    dotenv().ok();
    let dev = env::var("DATABASE_URL");
    let prod = ProjectDirs::from("", "Yourcase", "App").expect("No HOME dir found");
    dev.unwrap_or(prod.data_dir().to_str().unwrap().to_string())
}

impl Database {
    fn connect() -> SqliteConnection {
        SqliteConnection::establish(&DATABASE_PATH)
            .unwrap_or_else(|_| panic!("Error connecting to {}", *DATABASE_PATH))
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<SqliteConnection> {
        self.0.lock().unwrap()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self(Mutex::new(Database::connect()))
    }
}
