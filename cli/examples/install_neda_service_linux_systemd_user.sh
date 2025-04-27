#!/bin/bash

# make sure you have the user dir.
mkdir -p ~/.config/systemd/user/neda.service;

# make sure you have the neda cli installed.
if ! command -v neda &> /dev/null; then
    echo "Error: 'neda' CLI is not installed. Install it with: cargo install neda-cli" >&2
    exit 1
fi

NEDA_CLI=$(command -v neda);

# create the neda service file.
echo "
[Unit]
Description=Neda Prayer Times Service
After=network.target

[Service]
Type=simple
ExecStart=$NEDA_CLI start
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
" > ~/.config/systemd/user/neda.service/neda.service;

# reload the systemd daemon.
systemctl --user daemon-reload;

# enable and start the neda service.
systemctl --user enable --now neda.service;

echo "✅ Neda service installed and started successfully!"
echo "Check status with: systemctl --user status neda.service"
