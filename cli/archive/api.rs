use chrono::Datelike;
use neda_lib::{core::{
    config::{Config, GetType},
    prayers_times::PrayersTimesStuck, providers::{Provider, ProviderError},
}, providers::aladhan::AladhanProvider};

pub fn fetch_prayers_times(fetch_type: GetType, config: &Config) -> Result<PrayersTimesStuck, ProviderError> {
    let today = chrono::Local::now();
    let api_config = Config::new(
        today.year(),
        today.month(),
        today.day(),
        config.city.clone(),
        config.country.clone(),
        fetch_type,
    );

    let aladhan = AladhanProvider::new(api_config.clone());
    let prayers_times = aladhan.get_prayers_times(&api_config);

    match prayers_times {
        Ok(prayers_times) => Ok(prayers_times),
        Err(e) => {
            println!("error: {:?}", e);
            Err(e)
        }
    }
}
