use std::path::Path;

use neda_lib::{
    core::{config::Config, prayers_times::Prayers, providers::Provider},
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
        neda_lib::core::config::GetType::Month,
    );
    let aladhan = AladhanProvider::new(config.clone());
    let prayers_times = aladhan.get_prayers_times(&config).unwrap();
    // println!("prayers_times is: {:#?}", prayers_times);

    let db = PrayersTimesDB::new("lib/local.db".to_string());
    match db {
        Ok(mut db) => {
            match Path::new("lib/local.db").exists() {
                true => {
                    println!("local.db exists, not pushing to db");
                    let day_salawat = db.get_day_times(&prayers_times.from).unwrap();
                    let fjer = db.get_prayer_time(Prayers::Fajr, &prayers_times.from);
                    println!("fjer is: {:#?}", fjer);
                    println!("day salawat is: {:#?}", day_salawat);
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
