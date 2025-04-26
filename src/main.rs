use neda::{
    core::{config::Config, providers::Provider},
    providers::aladhan::AladhanProvider,
};

fn main() {
    let config = Config::new(
        2025,
        4,
        25,
        "ElOued".to_string(),
        "DZ".to_string(),
        neda::core::config::GetType::Month,
    );
    let aladhan = AladhanProvider::new(config.clone());
    let prayers_times = aladhan.get_prayers_times(&config).unwrap();
    println!("prayers_times is: {:#?}", prayers_times);
}

