use gtoml::{get, parse};
use std::{env, fs, path::Path, process::exit};

#[derive(Debug)]
pub struct ApiConfig {
    pub url: String,
}

#[derive(Debug)]
pub struct AdhanConfig {
    pub enabled: bool,
    pub file: String,
}

#[derive(Debug)]
pub struct Config {
    pub api: ApiConfig,
    pub adhan: AdhanConfig,
}

pub fn get_config() -> Config {
    let config_dir_path =
        env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap() + "/.config/neda");

    fs::create_dir_all(&config_dir_path).unwrap();

    let config_path = config_dir_path + "/config.toml";

    const DEFAULT_CONFIG: &str = include_str!("../../assets/templetes/config.toml");

    if !Path::new(&config_path).exists() {
        fs::write(&config_path, DEFAULT_CONFIG).unwrap();
    }

    let config: String = fs::read_to_string(&config_path).unwrap();
    let config = parse(&config).unwrap();

    Config {
        api: ApiConfig {
            url: match get(&config, "api.url").unwrap() {
                Some(value) => value.as_str().unwrap().to_string(),
                None => {
                    println!("api.url is not set in config.toml");
                    exit(1);
                }
            },
        },
        adhan: AdhanConfig {
            enabled: match get(&config, "adhan.enabled").unwrap() {
                Some(value) => value.as_bool().unwrap(),
                None => {
                    println!("api.url is not set in config.toml");
                    exit(1);
                }
            },
            file: match get(&config, "adhan.file").unwrap() {
                Some(value) => value.as_str().unwrap().to_string(),
                None => {
                    println!("api.url is not set in config.toml");
                    exit(1);
                }
            },
        },
    }
}
