use std::env;

use neda_cli::list::list;
use neda_lib::{client::config_reader::Config, storage::prayers_times_db::PrayersTimesDB};

enum Error {
    InvalidOption(String),
    TooManyArguments,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::load();

    if let Err(e) = &config {
        println!(
            "you have an error in your config please make sure you set all the fild corctly and try again: {:?}",
            e
        );
    };

    let mut db = PrayersTimesDB::new(config.unwrap().db.path).unwrap();

    let _ = match args.len() > 2 {
        true => Err(Error::TooManyArguments),
        false => match args[1].as_str() {
            "list" => {
                list(&mut db, args.get(2).unwrap_or(&"today".to_string()));
                Ok(())
            }
            "start" => {
                println!("start");
                Ok(())
            }
            _ => Err(Error::InvalidOption(args[1].to_string())),
        },
    };
}
