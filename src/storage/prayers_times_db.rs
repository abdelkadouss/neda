use crate::core::prayers_times::PrayersTimesStuck;

use super::{DB, Table};

pub struct PrayersTimesDB {
    db: DB,
}

impl PrayersTimesDB {
    pub fn new(db_path: String) -> Result<Self, rusqlite::Error> {
        let prayers_times_schema = r#"
            date TEXT NOT NULL PRIMARY KEY,
            fajr TEXT NOT NULL,
            dhuhr TEXT NOT NULL,
            asr TEXT NOT NULL,
            maghrib TEXT NOT NULL,
            isha TEXT NOT NULL
        "#
        .to_string();

        let prayers_times_table = Table::new(String::from("prayers_times"), prayers_times_schema);

        let db = DB::new(db_path, vec![prayers_times_table])?;
        Ok(PrayersTimesDB { db })
    }
}

impl PrayersTimesDB {
    pub fn push(&mut self, data: &PrayersTimesStuck) -> Result<(), rusqlite::Error> {
        let mut current_date = data.from;

        for prayers_times in data.prayers_times.iter() {
            // Create params that implement ToSql trait
            let date_str = current_date.format("%Y-%m-%d").to_string();
            let fajr_str = prayers_times.fajr.to_string();
            let dhuhr_str = prayers_times.dhuhr.to_string();
            let asr_str = prayers_times.asr.to_string();
            let maghrib_str = prayers_times.maghrib.to_string();
            let isha_str = prayers_times.isha.to_string();

            // Create a Vec of parameters that correctly implements ToSql
            let params: Vec<(&str, &dyn rusqlite::ToSql)> = vec![
                ("date", &date_str as &dyn rusqlite::ToSql),
                ("fajr", &fajr_str as &dyn rusqlite::ToSql),
                ("dhuhr", &dhuhr_str as &dyn rusqlite::ToSql),
                ("asr", &asr_str as &dyn rusqlite::ToSql),
                ("maghrib", &maghrib_str as &dyn rusqlite::ToSql),
                ("isha", &isha_str as &dyn rusqlite::ToSql),
            ];

            // Pass the params slice to the push method
            self.db.push(String::from("prayers_times"), &params)?;

            // Update the date for the next iteration
            current_date = current_date.checked_add_days(chrono::Days::new(1)).unwrap();
        }

        Ok(())
    }
}
