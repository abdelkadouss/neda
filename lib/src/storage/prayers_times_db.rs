use chrono::NaiveDate;

use crate::core::prayers_times::{Prayers, PrayersTimes, PrayersTimesStuck};

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

    pub fn get_day_times(&self, date: &NaiveDate) -> Result<PrayersTimes, rusqlite::Error> {
        // Create a properly parameterized query
        let sql = "SELECT fajr, dhuhr, asr, maghrib, isha FROM prayers_times WHERE date = ?";
        let date_str = date.format("%Y-%m-%d").to_string();

        // Prepare and execute the query with parameters
        let mut stmt = self.db.conn.prepare(sql)?;
        let mut rows = stmt.query(&[&date_str as &dyn rusqlite::ToSql])?;

        // Get the first row (should be only one since date is the primary key)
        if let Some(row) = rows.next()? {
            // Since we can't create a rusqlite::Error::InvalidColumnType directly,
            // let's use a different approach to handle parsing errors

            // Helper function to parse a time string or return an error
            let parse_time = |idx: usize, time_str: String| -> Result<_, rusqlite::Error> {
                time_str.parse().map_err(|_| {
                    rusqlite::Error::FromSqlConversionFailure(
                        idx,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid time format",
                        )),
                    )
                })
            };

            // Convert each field with proper type specification
            let prayers_times = PrayersTimes {
                fajr: parse_time(0, row.get::<_, String>(0)?)?,
                dhuhr: parse_time(1, row.get::<_, String>(1)?)?,
                asr: parse_time(2, row.get::<_, String>(2)?)?,
                maghrib: parse_time(3, row.get::<_, String>(3)?)?,
                isha: parse_time(4, row.get::<_, String>(4)?)?,
            };
            Ok(prayers_times)
        } else {
            // Return an error if no prayer times found for the given date
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn get_prayer_time(
        &self,
        prayer: Prayers,
        date: &NaiveDate,
    ) -> Result<String, rusqlite::Error> {
        // Determine the column name based on the prayer type
        let column = match prayer {
            Prayers::Fajr => "fajr",
            Prayers::Dhuhr => "dhuhr",
            Prayers::Asr => "asr",
            Prayers::Maghrib => "maghrib",
            Prayers::Isha => "isha",
        };

        // Create a parameterized query to get just the requested prayer time
        let sql = format!("SELECT {} FROM prayers_times WHERE date = ?", column);
        let date_str = date.format("%Y-%m-%d").to_string();

        // Execute the query
        let mut stmt = self.db.conn.prepare(&sql)?;
        let mut rows = stmt.query(&[&date_str as &dyn rusqlite::ToSql])?;

        // Get the first row
        if let Some(row) = rows.next()? {
            let time = row.get::<_, String>(0)?;
            Ok(time)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
