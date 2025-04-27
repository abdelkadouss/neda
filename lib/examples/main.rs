use chrono::Datelike;
use neda_lib::{
    core::{config::Config, prayers_times::Prayers, providers::Provider},
    providers::aladhan::AladhanProvider,
    storage::prayers_times_db::PrayersTimesDB,
};

fn main() {
    let today = chrono::Local::now();
    let config = Config::new(
        today.year(),
        today.month(),
        today.day(),
        "makah".to_string(),
        "SAU".to_string(),
        neda_lib::core::config::GetType::Month,
    );
    let aladhan = AladhanProvider::new(config.clone());
    let prayers_times = aladhan.get_prayers_times(&config).unwrap();
    // println!("prayers_times is: {:#?}", prayers_times);

    let db = PrayersTimesDB::new("lib/local.db".to_string());
    match db {
        Ok(mut db) => {
            match db.push(&prayers_times) {
                Ok(_) => {
                    println!("ignore pushing to the db");
                }
                _ => {
                    println!("pushing to the db");
                }
            };

            let day_salawat = db.get_day_times(&prayers_times.from).unwrap();
            let fjer = db.get_prayer_time(Prayers::Fajr, &prayers_times.from);
            println!("fjer is: {:#?}", fjer);
            println!("day salawat is: {:#?}", day_salawat);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }
}
