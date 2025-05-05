<div align="center"> بسم الله الرحمن الرحيم </div> 

# WORNING
🚧 this app is still in development and is not really stable yet 🚧

# Neda

Neda-cli lightweight cli app for prayer times (show, notify, adhan) and is a part (implementation) of [Neda](https://github.com/abdelkadouss/neda) project a free and open source cross platform - insha'Allah - incha'Allah. salat (prayers) times app and library.

## Installation

### Clients (cli)

You can install the Neda cli with cargo:

```sh
cargo install neda-cli
```

todo: installing via `nix package manager`.

## Configuration

after the first time you run the cli neda should write the default configuration file to the `~/.config/neda/config.toml` file.
in this file you should set the place you are living the `city` and the `country` fields then set the database path that the place where you want Neda to store priers times in, then the `adhan.file` field is the field where you can set the adhan you want Neda to run (an mp3 file)

## Usage

```sh
neda # and the adhan service should be running...🌻

```

### Adhan and Notification Service
you have to add this to your start at login list to start the adhan and notification service in the background when you open the machine.

#### Linux users

if you use a destro that use systemd do the use the `scripts/install_neda_service_linux_systemd_user.sh` script to add the neda service.

so you can incha'Allah run this to directly, this will fetch the script and run it.

```sh
curl -s https://raw.githubusercontent.com/abdelkadouss/neda/refs/heads/master/cli/scripts/install_neda_service_linux_systemd_user.sh | sh
```

or for manual install, install it and run:

```sh
sh install_neda_service_linux_systemd_user.sh
```

you should see:

```sh
# =>✅ Neda service installed and started successfully!
# =>Check status with: systemctl --user status neda.service
```

#### OSX (MacOS) users

use the secript example under the scripts folder under name `scripts/install_neda_service_osx_user.sh` and run it.

so you can incha'Allah run this to directly, this will fetch the script and run it.

```sh
curl -s https://raw.githubusercontent.com/abdelkadouss/neda/refs/heads/master/cli/scripts/install_neda_service_osx_user.sh | sh
```

or for manual install, install it and run:

```sh
sh install_neda_service_osx_user.sh
```

you should see:

```sh
# =>✅ Neda service installed and started successfully!
# =>Check status with: systemctl --user status neda.service
```

and you should see a notification pop up that tell you neda added to the ligin items or some thing like that incha'Allah.

## Issues and feature requests

If you find a bug or want to request a feature, please open an issue on the [GitHub repository](https://github.com/abdelkadouss/neda/issues).

## Contributing

Contributions are welcome! [the repo](https://github.com/abdelkadouss/neda) is open to pull requests.

### License
you can use it under the terms of either the [MIT](https://choosealicense.com/licenses/mit/) license or the [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/) license.
