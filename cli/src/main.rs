use chrono::{Datelike, NaiveDate, NaiveTime, TimeDelta, Timelike};
use neda_lib::{
    client::config_reader::Config,
    core::{
        config::{Config as apiConfig, GetType},
        prayers_times::PrayersTimes,
        providers::Provider,
    },
    providers::aladhan::AladhanProvider,
    sound::Adhan,
    storage::{Error, prayers_times_db::PrayersTimesDB},
};
use std::{thread::sleep, time::Duration};

fn main() {
    // Use a loop instead of recursive calls to avoid stack overflow
    //

    let config = Config::load().unwrap();

    loop {
        let db_path = &config.db.path;
        let mut db = Err(Error::InvalidQuery);
        let mut attempt = 0;
        while attempt < 3 && db.is_err() {
            db = PrayersTimesDB::new(db_path.clone());
            attempt += 1;
        }

        match db {
            Ok(mut ready_db) => {
                let today_prayers_time = get_today_prayers_time_from_db(&ready_db);
                match today_prayers_time {
                    Ok(today_prayers_time) => {
                        let next_prayer_time =
                            calculate_next_prayers_time(&ready_db, &today_prayers_time);

                        if next_prayer_time.is_none() {
                            eprintln!("No next prayer time found");
                            // Add a small delay before retrying to prevent CPU spinning
                            sleep(Duration::from_secs(60 * 5));
                            continue;
                        }

                        let next_prayer_time = next_prayer_time.unwrap();

                        loop {
                            let now = chrono::Local::now().time();
                            let now = NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap();

                            if next_prayer_time.lt(&now) || next_prayer_time.eq(&now) {
                                if (now - next_prayer_time).abs() > TimeDelta::seconds(300) {
                                    break;
                                }

                                let adhan = Adhan::new(config.adhan.file.clone());
                                adhan.play();
                                break;
                            }

                            sleep(Duration::from_secs(60));
                        }
                        // Continue the loop instead of recursive call
                    }
                    Err(_) => {
                        if let Err(e) = update_db(&mut ready_db, &config) {
                            eprintln!("Failed to update database: {:?}", e);
                            // Add a small delay before retrying to prevent CPU spinning
                            sleep(Duration::from_secs(10));
                        }
                        // Continue the loop instead of recursive call
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open database after multiple attempts: {:?}", e);
                // Add a delay before retrying to prevent CPU spinning
                sleep(Duration::from_secs(120));
                // Continue the loop instead of recursive call
            }
        }
    }
}

fn get_today_prayers_time_from_db(db: &PrayersTimesDB) -> Result<PrayersTimes, Error> {
    let today = chrono::Local::now();
    let today_date = &NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();
    db.get_day_times(today_date)
}

fn get_prayers_time_for_date(db: &PrayersTimesDB, date: &NaiveDate) -> Result<PrayersTimes, Error> {
    db.get_day_times(date)
}

fn update_db(db: &mut PrayersTimesDB, global_config: &Config) -> Result<(), Error> {
    let today = chrono::Local::now();
    let config = apiConfig::new(
        today.year(),
        today.month(),
        today.day(),
        global_config.api.city.clone(),
        global_config.api.country.clone(),
        GetType::Month,
    );
    let provider = AladhanProvider::new(config.clone());
    let prayers_times = provider.get_prayers_times(&config).unwrap();
    db.overwrite(&prayers_times)?;
    Ok(())
}

fn calculate_next_prayers_time(
    db: &PrayersTimesDB,
    today_prayers_time: &PrayersTimes,
) -> Option<NaiveTime> {
    let now = chrono::Local::now().time();

    // Create an array of prayer times and find the next prayer time
    let next_prayer = [
        today_prayers_time.fajr,
        today_prayers_time.dhuhr,
        today_prayers_time.asr,
        today_prayers_time.maghrib,
        today_prayers_time.isha,
    ]
    .iter()
    .filter(|&&time| now < time)
    .min()
    .copied();

    // If there's a prayer time today, calculate duration until that time
    if let Some(next_time) = next_prayer {
        Some(next_time)
    } else {
        // If no more prayers today, calculate time until tomorrow's Fajr
        // Create a new date for tomorrow
        let today = chrono::Local::now();
        let tomorrow = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
            .unwrap()
            .succ_opt()
            .unwrap();

        // Get tomorrow's prayer times
        match get_prayers_time_for_date(db, &tomorrow) {
            Ok(prayers) => Some(prayers.fajr),
            Err(_) => {
                // If we can't get tomorrow's prayers, use a fallback time
                // This is a placeholder. In a real app, you might want to handle this differently
                // sleep for a minites and try again
                None
            }
        }
    }
}
