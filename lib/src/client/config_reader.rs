use gtoml::{get, parse};
use std::{env, fs, path::Path};

#[derive(Debug)]
pub struct ApiConfig {
    pub city: String,
    pub country: String,
}

#[derive(Debug)]
pub struct AdhanConfig {
    pub enabled: bool,
    pub file: String,
}

#[derive(Debug)]
pub struct DbConfig {
    pub path: String,
}

#[derive(Debug)]
pub struct Config {
    pub api: ApiConfig,
    pub adhan: AdhanConfig,
    pub db: DbConfig,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    ParseError(String),
    MissingField(String),
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir_path = match env::var("XDG_CONFIG_HOME") {
            Ok(path) => path + "/neda",
            Err(_) => env::var("HOME").unwrap() + "/.config/neda",
        };

        let data_dir_path = match env::var("XDG_DATA_HOME") {
            Ok(path) => path + "/neda",
            Err(_) => env::var("HOME").unwrap() + "/.local/share/neda",
        };

        fs::create_dir_all(&config_dir_path).map_err(ConfigError::Io)?;
        fs::create_dir_all(&data_dir_path).map_err(ConfigError::Io)?;

        let config_path = config_dir_path + "/config.toml";

        const DEFAULT_CONFIG: &str = include_str!("../../assets/templetes/config.toml");

        if !Path::new(&config_path).exists() {
            fs::write(&config_path, DEFAULT_CONFIG).map_err(ConfigError::Io)?;
        }

        let config: String = fs::read_to_string(&config_path).map_err(ConfigError::Io)?;
        let config = parse(&config).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(Config {
            api: ApiConfig {
                city: match get(&config, "api.city")
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
                {
                    Some(value) => value
                        .as_str()
                        .ok_or_else(|| {
                            ConfigError::ParseError("api.city is not a string".to_string())
                        })?
                        .to_string(),
                    None => {
                        return Err(ConfigError::MissingField(
                            "api.city is not set in config.toml".to_string(),
                        ));
                    }
                },
                country: match get(&config, "api.country")
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
                {
                    Some(value) => value
                        .as_str()
                        .ok_or_else(|| {
                            ConfigError::ParseError("api.country is not a string".to_string())
                        })?
                        .to_string(),
                    None => {
                        return Err(ConfigError::MissingField(
                            "api.country is not set in config.toml".to_string(),
                        ));
                    }
                },
            },
            adhan: AdhanConfig {
                enabled: match get(&config, "adhan.enabled")
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
                {
                    Some(value) => value.as_bool().ok_or_else(|| {
                        ConfigError::ParseError("adhan.enabled is not a boolean".to_string())
                    })?,
                    None => {
                        return Err(ConfigError::MissingField(
                            "adhan.enabled is not set in config.toml".to_string(),
                        ));
                    }
                },
                file: match get(&config, "adhan.file")
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
                {
                    Some(value) => value
                        .as_str()
                        .ok_or_else(|| {
                            ConfigError::ParseError("adhan.file is not a string".to_string())
                        })?
                        .to_string(),
                    None => {
                        return Err(ConfigError::MissingField(
                            "adhan.file is not set in config.toml".to_string(),
                        ));
                    }
                },
            },
            db: DbConfig {
                path: match get(&config, "db.path")
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
                {
                    Some(value) => value
                        .as_str()
                        .ok_or_else(|| {
                            ConfigError::ParseError("db.path is not a string".to_string())
                        })?
                        .to_string(),
                    None => {
                        return Err(ConfigError::MissingField(
                            "db.path is not set in config.toml".to_string(),
                        ));
                    }
                },
            },
        })
    }
}
