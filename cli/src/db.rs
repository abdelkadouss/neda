use neda_lib::storage::Error;

use neda_lib::{core::config::{Config, GetType}, storage::prayers_times_db::PrayersTimesDB};

use crate::api::fetch_prayers_times;

pub fn update_db(db: &mut PrayersTimesDB, config: &Config) -> Result<(), Error> {
    let prayer_times = fetch_prayers_times(GetType::Month, config);
    match prayer_times {
        Ok(prayer_times) => {
            let res = db.overwrite(&prayer_times);
            match res {
                Ok(_) => {
                    Ok(())
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("error: {:?}", e);
            Err(Error::InvalidQuery)
        }
    }
}
