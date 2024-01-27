pub mod models;
pub mod schema;
use chrono::{Datelike, Timelike, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use sha2::{Digest, Sha256};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn generate_short(url: &String) -> String {
    // Get the current date and time
    let now = Utc::now();

    // Format the date and time to a string and slice to 8 characters
    let date_time_str = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond()
    )
    .chars()
    .collect::<String>();

    // Concatenate the string and date+time
    let combined_data = format!("{}{}", url, date_time_str);

    // Create a SHA-256 hasher
    let mut hasher = Sha256::new();

    // Update the hasher with the combined data
    hasher.update(&combined_data);

    // Get the checksum
    let checksum = format!("{:x}", hasher.finalize())[..8].to_string();

    checksum
}
