use chrono::{Datelike, NaiveDate, NaiveTime};

pub enum Prayers {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

#[derive(Debug)]
pub struct PrayersTimes {
    pub fajr: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
}

#[derive(Debug)]
pub struct PrayersTimesStuck {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub prayers_times: Vec<PrayersTimes>,
}

impl Default for PrayersTimes {
    fn default() -> Self {
        Self::new(
            NaiveTime::default(),
            NaiveTime::default(),
            NaiveTime::default(),
            NaiveTime::default(),
            NaiveTime::default(),
        )
    }
}

impl PrayersTimes {
    pub fn new(
        fajr: NaiveTime,
        dhuhr: NaiveTime,
        asr: NaiveTime,
        maghrib: NaiveTime,
        isha: NaiveTime,
    ) -> PrayersTimes {
        PrayersTimes {
            fajr,
            dhuhr,
            asr,
            maghrib,
            isha,
        }
    }
}

impl PrayersTimesStuck {
    pub fn new(from: NaiveDate, to: NaiveDate, prayers_times: Vec<PrayersTimes>) -> PrayersTimesStuck {
        PrayersTimesStuck {
            from,
            to,
            prayers_times,
        }
    }

    pub fn get(&self, prayers: Prayers, date: NaiveDate) -> Option<&NaiveTime> {
        let index = date.num_days_from_ce() - self.from.num_days_from_ce();
        let index = index as u32;
        if index >= self.prayers_times.len() as u32 {
            None
        } else {
            let times: Option<&NaiveTime> = match &self.prayers_times.get(index as usize) {
                Some(times) => match prayers {
                    Prayers::Fajr => Some( &times.fajr ),
                    Prayers::Dhuhr => Some( &times.dhuhr ),
                    Prayers::Asr => Some( &times.asr ),
                    Prayers::Maghrib => Some( &times.maghrib ),
                    Prayers::Isha => Some( &times.isha ),
                },
                None => None,
            };
            times
        }
    }
}
