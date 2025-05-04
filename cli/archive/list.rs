use std::fmt::Result;

use chrono::{Datelike, NaiveDate};
use neda_lib::{core::{config::Config, prayers_times::{PrayersTimes, PrayersTimesStuck}}, storage::prayers_times_db::PrayersTimesDB};

use crate::db::update_db;

enum ReturnType {
    Stuck(PrayersTimesStuck),
    Single(PrayersTimes),
}

pub fn list(prayers_times_db: &mut PrayersTimesDB, option: &str, config: Config) {
    let today = chrono::Local::now();
    let today_date = &NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    let prayers_times: Result<ReturnType, _> = match option {
        "today" => ReturnType::Stuck(prayers_times_db.get_day_times(today_date)),
        "month" => ReturnType::Single(prayers_times_db.get_month_times(today_date)),
        _ => {
            println!("invalid (or unsupported) option");
            Err("invalid (or unsupported) option")
        }
    };
}
