use chrono::{DateTime, Local, TimeZone};

pub mod commands;
pub mod database;

pub fn epoch_now() -> i32 {
    chrono::Utc::now()
        .timestamp()
        .try_into()
        .expect("Epoch time past i32")
}

pub fn local_time_from_epoch(epoch: i32) -> DateTime<Local> {
    chrono::Local.timestamp_opt(epoch.into(), 0).unwrap()
}
