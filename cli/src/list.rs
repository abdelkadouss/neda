use chrono::{Datelike, NaiveDate};
use neda_lib::{core::config::Config, storage::prayers_times_db::PrayersTimesDB};

use crate::db::update_db;

pub fn list(prayers_times_db: &mut PrayersTimesDB, option: &str, config: Config) {
    let today = chrono::Local::now();
    let today_date = &NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    match option {
        "today" => {
            let today_prayers_times = prayers_times_db.get_day_times(today_date);
            if let Ok(today_prayers_times) = today_prayers_times {
                println!("today prayers times: {:#?}", today_prayers_times);
            } else {
                let res = update_db(prayers_times_db, &config);
                match res {
                    Ok(_) => {
                        let today_prayers_times = prayers_times_db.get_day_times(today_date);
                        if let Ok(today_prayers_times) = today_prayers_times {
                            println!("today prayers times: {:#?}", today_prayers_times);
                        } else {
                            todo!()
                        }
                    }
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
        }
        "month" => {
            println!("month");
        }
        _ => {
            println!("invalid (or unsupported) option");
        }
    }
}
