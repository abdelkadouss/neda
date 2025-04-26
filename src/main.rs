use std::path::Path;

use neda::{
    core::{config::Config, providers::Provider},
    providers::aladhan::AladhanProvider,
    storage::prayers_times_db::PrayersTimesDB,
};

fn main() {
    let config = Config::new(
        2025,
        4,
        25,
        "ElOued".to_string(),
        "DZ".to_string(),
        neda::core::config::GetType::Month,
    );
    let aladhan = AladhanProvider::new(config.clone());
    let prayers_times = aladhan.get_prayers_times(&config).unwrap();
    // println!("prayers_times is: {:#?}", prayers_times);

    let db = PrayersTimesDB::new("local.db".to_string());
    match db {
        Ok(mut db) => {
            match Path::new("local.db").exists() {
                true => {
                    println!("local.db exists, not pushing to db");
                }
                _ => {
                    db.push(&prayers_times).unwrap();
                }
            };
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }
}
