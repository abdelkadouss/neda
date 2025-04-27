<div align="center"> بسم الله الرحمن الرحيم </div> 

# Neda

Neda is a Rust library for getting prayer times from multiple providers.
is good to make you own prayer times app really quickly and easily incha'Allah.
it is a part of [Neda](https://github.com/abdelkadess/neda) project a free and open source cross platform - insha'Allah - salat (prayers) times app and library.

## Installation

### Clients (cli, gui)

You can install the Neda cli with cargo:

```sh
cargo install neda-cli
```

### Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
neda = "0.1"
```

### Features

default = ["aladhan-provider", "sqlite-storage"]
aladhan-provider # default api provider.
sqlite-storage # storage module, a sqlite db for offline access.
client # utilities to help you make your own neda client (including a config_reader and some stuff).


## Usage

```rust
use chrono::Datelike;
use neda_lib::{
    core::{config::Config, prayers_times::Prayers, providers::Provider},
    providers::aladhan::AladhanProvider,
    storage::prayers_times_db::PrayersTimesDB,
};

fn main() {
    // Configure the options for fetching prayer times (location, country, and retrieval type: today, date, month, or year)
    let today = chrono::Local::now();
    let config = Config::new(
        today.year(),
        today.month(),
        today.day(),
        "makah".to_string(),
        "SAU".to_string(),
        neda_lib::core::config::GetType::Month,
    );

    // Use the provider of your choice. Here, we use the Aladhan provider (the default implementation provided by Neda's original developer).
    // You can also use Neda's library to implement your own provider or use community-provided implementations (if exits).
    // Check `lib/providers/aladhan` for reference.
    let aladhan = AladhanProvider::new(config.clone());
    let prayers_times = aladhan.get_prayers_times(&config).unwrap();

    // Optional: Uncomment to inspect the prayer times.
    // println!("prayers_times is: {:#?}", prayers_times);

    // now you can store the prayer times (using a sqlite db) for offline access using the storage module.
    let db = PrayersTimesDB::new("lib/local.db".to_string());
    // You can examine the `lib/local.db` file afterward.

    match db {
        Ok(mut db) => {
            // this how push the prayers times to the db incha'Allah.
            match db.push(&prayers_times) {
                Ok(_) => {
                    println!("ignore pushing to the db");
                }
                _ => {
                    println!("pushing to the db");
                }
            };

            // now you can get the day prayers times from the db really easy incha'Allah using the get methods.
            let day_salawat = db.get_day_times(&prayers_times.from).unwrap();
            // specific prayer in specific date.
            let fjer = db.get_prayer_time(Prayers::Fajr, &prayers_times.from);
            println!("fjer is: {:#?}", fjer);
            println!("day prayeris : {:#?}", day_salawat);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }
}
```

> Note: you can find this example in `lib/examples/main.rs` file, and you can run it with `cargo run --example main`.

> Note: you can find an client implementation under `cli` crate and a gui soon incha'Allah.

### License
you can use this library under the terms of either the [MIT](https://choosealicense.com/licenses/mit/) license or the [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/) license.
