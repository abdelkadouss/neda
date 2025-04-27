<div align="center"> بسم الله الرحمن الرحيم </div> 

# Neda

Neda-cli lightweight cli app for prayer times (show, notify, adhan) and is a part (implementation) of [Neda](https://github.com/abdelkadess/neda) project a free and open source cross platform - insha'Allah - incha'Allah. salat (prayers) times app and library.

## Installation

### Clients (cli)

You can install the Neda cli with cargo:

```sh
cargo install neda-cli
```

todo: installing via `nix package manager`.

## Usage

```sh
neda --help # print help message

neda list #<options> # list prayer times (default: today)

neda show #<options> # equal to list command

neda start # start adhan and notification service in the background

```

### Adhan and Notification Service
you have to add this to your start at login list to start the adhan and notification service in the background when you open the machine.

#### Linux users

if you use a destro that use systemd do the next:

```sh
sudo systemctl --user enable --now neda-adhan.service
sudo systemctl --user enable --now neda-notify.service
```

#### OSX (MacOS) users

use the secript example examples folder under name `neda-osx-start-at-login.sh` and run it.
```sh
sh neda-osx-start-at-login.sh
```

you should see a notification pop up that tell you neda added to the ligin items or some thing like that incha'Allah.

### License
you can use it under the terms of either the [MIT](https://choosealicense.com/licenses/mit/) license or the [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/) license.
