pub mod commands;
pub mod database;


pub fn epoch_now() -> i32 {
    chrono::Utc::now().timestamp().try_into().expect("Epoch time past i32")
}