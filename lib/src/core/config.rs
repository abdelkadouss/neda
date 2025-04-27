use chrono::{Datelike, Local};

pub enum GetType {
    Today,
    Date,
    Month,
    Year,
}

impl GetType {
    fn clone(&self) -> GetType {
        match self {
            GetType::Today => GetType::Today,
            GetType::Date => GetType::Date,
            GetType::Month => GetType::Month,
            GetType::Year => GetType::Year,
        }
    }
}

pub struct Config {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub city: String,
    pub country: String,
    pub get_type: GetType,
}

impl Default for Config {
    fn default() -> Self {
        let today = Local::now().date_naive();
        Self::new(today.year(), today.month(), today.day(), String::from("Maka"), String::from("SAU"), GetType::Month)
    }
}

impl Config {
    pub fn new(year: i32, month: u32, day: u32, city: String, country: String, get_type: GetType) -> Config {
        Config {
            year,
            month,
            day,
            city,
            country,
            get_type,
        }
    }

    pub fn clone(&self) -> Config {
        Config {
            year: self.year,
            month: self.month,
            day: self.day,
            city: self.city.clone(),
            country: self.country.clone(),
            get_type: self.get_type.clone(),
        }
    }
}
