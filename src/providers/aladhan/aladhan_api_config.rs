pub struct AladhanEndpoint {
    pub today: String,
    pub date: String,
    pub month: String,
    pub year: String,
}

pub struct AladhanApiConfig {
    pub base_url: String,
    pub api_version: String,
    pub endpoint: AladhanEndpoint,
}

pub fn get_aladhan_api_config() -> AladhanApiConfig {
    let aladhan_api_and_endpoint = AladhanEndpoint {
        today: String::from("timingsByCity"),
        date: String::from("timingsByCity"),
        month: String::from("calendarByCity/from/{from}/to/{to}"),
        year: String::from("calendarByCity/from/{start}/to/{end}"),
    };

    AladhanApiConfig {
        base_url: String::from("https://api.aladhan.com"),
        api_version: String::from("1"),
        endpoint: aladhan_api_and_endpoint,
    }
}
