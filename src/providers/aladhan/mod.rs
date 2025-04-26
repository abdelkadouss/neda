use crate::core::config::{Config, GetType};
use crate::core::prayers_times::{PrayersTimes, PrayersTimesStuck};
use crate::core::providers::{Provider, ProviderError};
use chrono::{Datelike, NaiveDate, NaiveTime};
use serde_json::Value;
use std::str::FromStr;
mod aladhan_api_config;

#[cfg(test)]
mod test;

pub struct AladhanProvider {
    pub config: Config,
}

impl AladhanProvider {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Provider for AladhanProvider {
    fn get_prayers_times(&self, config: &self::Config) -> Result<PrayersTimesStuck, ProviderError> {
        let aladhan_api_config = aladhan_api_config::get_aladhan_api_config();
        match config.get_type {
            GetType::Today => {
                let url = format!(
                    "{}/v{}/{}/{}-{}-{}?city={}&country={}",
                    aladhan_api_config.base_url,
                    aladhan_api_config.api_version,
                    aladhan_api_config.endpoint.today,
                    config.day,
                    config.month,
                    config.year,
                    config.city,
                    config.country
                );

                match reqwest::blocking::get(url) {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::OK => {
                            match response.text() {
                                Ok(body) => {
                                    match serde_json::from_str::<Value>(&body) {
                                        Ok(v) => {
                                            // Extract prayer times from JSON, with better error handling
                                            let fajr = v["data"]["timings"]["Fajr"]
                                                .as_str()
                                                .ok_or(ProviderError::InvalidResponse)?;
                                            let dhuhr = v["data"]["timings"]["Dhuhr"]
                                                .as_str()
                                                .ok_or(ProviderError::InvalidResponse)?;
                                            let asr = v["data"]["timings"]["Asr"]
                                                .as_str()
                                                .ok_or(ProviderError::InvalidResponse)?;
                                            let maghrib = v["data"]["timings"]["Maghrib"]
                                                .as_str()
                                                .ok_or(ProviderError::InvalidResponse)?;
                                            let isha = v["data"]["timings"]["Isha"]
                                                .as_str()
                                                .ok_or(ProviderError::InvalidResponse)?;

                                            // Parse time strings into NaiveTime
                                            let prayers_times = PrayersTimesStuck::new(
                                                NaiveDate::from_ymd_opt(
                                                    config.year,
                                                    config.month,
                                                    config.day,
                                                )
                                                .unwrap(),
                                                NaiveDate::from_ymd_opt(
                                                    config.year,
                                                    config.month,
                                                    config.day,
                                                )
                                                .unwrap(),
                                                vec![PrayersTimes::new(
                                                    NaiveTime::from_str(fajr).map_err(|_| {
                                                        ProviderError::InvalidResponse
                                                    })?,
                                                    NaiveTime::from_str(dhuhr).map_err(|_| {
                                                        ProviderError::InvalidResponse
                                                    })?,
                                                    NaiveTime::from_str(asr).map_err(|_| {
                                                        ProviderError::InvalidResponse
                                                    })?,
                                                    NaiveTime::from_str(maghrib).map_err(|_| {
                                                        ProviderError::InvalidResponse
                                                    })?,
                                                    NaiveTime::from_str(isha).map_err(|_| {
                                                        ProviderError::InvalidResponse
                                                    })?,
                                                )],
                                            );
                                            Ok(prayers_times)
                                        }
                                        Err(_) => Err(ProviderError::InvalidResponse),
                                    }
                                }
                                Err(_) => Err(ProviderError::InvalidResponse),
                            }
                        }
                        _ => Err(ProviderError::InvalidResponse),
                    },
                    Err(_) => Err(ProviderError::ConnectionError),
                }
            }
            GetType::Date => Err(ProviderError::UnsupportedOperation),
            GetType::Month => {
                let from_date =
                    chrono::NaiveDate::from_ymd_opt(config.year, config.month, config.day)
                        .ok_or(ProviderError::InvalidResponse)?;

                let to_date = from_date
                    .checked_add_days(chrono::Days::new(30))
                    .ok_or(ProviderError::InvalidResponse)?;

                let mut month_endpoint = aladhan_api_config.endpoint.month;

                // Replace placeholders in the endpoint
                month_endpoint = month_endpoint.replace(
                    "{from}",
                    &format!(
                        "{}-{}-{}",
                        from_date.day(),
                        from_date.month(),
                        from_date.year()
                    ),
                );

                month_endpoint = month_endpoint.replace(
                    "{to}",
                    &format!("{}-{}-{}", to_date.day(), to_date.month(), to_date.year()),
                );

                let url = format!(
                    "{}/v{}/{}?city={}&country={}",
                    aladhan_api_config.base_url,
                    aladhan_api_config.api_version,
                    month_endpoint,
                    config.city,
                    config.country
                );

                fn parse_day_times(
                    day_times: &Value,
                    time: &str,
                ) -> Result<NaiveTime, ProviderError> {
                    let salat_time = day_times[time]
                        .as_str()
                        .ok_or(ProviderError::InvalidResponse)?;

                    let salat_time = salat_time.split(" ").collect::<Vec<&str>>();
                    let salat_time = salat_time.first().ok_or(ProviderError::InvalidResponse)?;
                    let salat_time = NaiveTime::from_str(salat_time)
                        .map_err(|_| ProviderError::InvalidResponse)?;

                    Ok(salat_time)
                }

                match reqwest::blocking::get(url) {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::OK => match response.text() {
                            Ok(body) => match serde_json::from_str::<Value>(&body) {
                                Ok(v) => {
                                    // First, collect all the prayer times
                                    let mut prayer_times_vec = Vec::new();

                                    match v["data"].as_array() {
                                        Some(times_array) => {
                                            for day_times in times_array {
                                                let day_times = &day_times["timings"];

                                                let fajr = parse_day_times(day_times, "Fajr")?;
                                                let dhuhr = parse_day_times(day_times, "Dhuhr")?;
                                                let asr = parse_day_times(day_times, "Asr")?;
                                                let maghrib =
                                                    parse_day_times(day_times, "Maghrib")?;
                                                let isha = parse_day_times(day_times, "Isha")?;

                                                let prayer_times = PrayersTimes::new(
                                                    fajr,
                                                    dhuhr,
                                                    asr,
                                                    maghrib,
                                                    isha,
                                                );
                                                prayer_times_vec.push(prayer_times);
                                            }
                                        }
                                        _ => return Err(ProviderError::InvalidResponse),
                                    };

                                    // Create a new PrayersTimesStuck with the collected data
                                    let prayers_times = PrayersTimesStuck::new(
                                        from_date,
                                        to_date,
                                        prayer_times_vec,
                                    );

                                    Ok(prayers_times)
                                }
                                Err(_) => Err(ProviderError::InvalidResponse),
                            },
                            Err(_) => Err(ProviderError::InvalidResponse),
                        },
                        _ => Err(ProviderError::InvalidResponse),
                    },
                    Err(_) => Err(ProviderError::ConnectionError),
                }
            }
            GetType::Year => Err(ProviderError::UnsupportedOperation),
        }
    }
}
