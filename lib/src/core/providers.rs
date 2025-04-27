use crate::core::config::Config;
use crate::core::prayers_times::PrayersTimesStuck;

#[derive(Debug)]
pub enum ProviderError {
    NetworkError,
    PermissionError,
    InvalidConfig,
    InvalidDate,
    InvalidCity,
    InvalidCountry,
    UnknownError,
    InvalidResponse,
    TimeParseError,
    UnsupportedOperation,
    MissingField,
    ParseError,
    ConnectionError,
}

pub trait Provider {
    fn get_prayers_times(&self, config: &Config) -> Result<PrayersTimesStuck, ProviderError>;
}
