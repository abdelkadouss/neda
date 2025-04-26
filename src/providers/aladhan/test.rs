use chrono::{NaiveDate, NaiveTime, Timelike};

use crate::core::{
    config::{Config, GetType},
    prayers_times::{Prayers, PrayersTimesStuck},
    providers::{Provider, ProviderError},
};

use super::AladhanProvider;

#[test]
fn provider_today() -> Result<(), ProviderError> {
    // Setup test config
    let config = Config {
        year: 2025,
        month: 4,
        day: 25,
        city: "ElOued".to_string(),  // Fixed typo in city name
        country: "DZ".to_string(),
        get_type: GetType::Today,
    };

    let provider = AladhanProvider::new(config);

    let config = Config {
        year: 2025,
        month: 4,
        day: 25,
        city: "ElOued".to_string(),  // Fixed typo in city name
        country: "DZ".to_string(),
        get_type: GetType::Today,
    };
    
    // Get prayer times
    let prayers_times = provider.get_prayers_times(&config)?;
    
    // Verify date range
    let expected_date = NaiveDate::from_ymd_opt(2025, 4, 25)
        .expect("Invalid test date");
    assert_eq!(prayers_times.from, expected_date);
    assert_eq!(prayers_times.to, expected_date);
    
    // Verify Fajr time
    let fajr_time = prayers_times.get(Prayers::Fajr, expected_date)
        .expect("Fajr time not found");
    
    // Create expected time for comparison
    let expected_fajr = NaiveTime::from_hms_opt(4, 49, 0)
        .expect("Invalid test time");
    assert_eq!(fajr_time.hour(), expected_fajr.hour());
    assert_eq!(fajr_time.minute(), expected_fajr.minute());
    
    Ok(())
}
