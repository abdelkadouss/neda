#!/bin/bash

set -e  # Exit on error

# Check if 'neda' CLI is installed
if ! command -v neda &> /dev/null; then
    echo "Error: 'neda' CLI not found. Install it first (e.g., 'cargo install neda-cli')." >&2
    exit 1
fi

# Ensure LaunchAgents directory exists
mkdir -p ~/Library/LaunchAgents

# Path to the plist file
PLIST_PATH="$HOME/Library/LaunchAgents/com.neda.plist"

# Create the launchd plist file
cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.neda</string>
    <key>ProgramArguments</key>
    <array>
        <string>$(command -v neda)</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/neda.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/neda.err</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>$PATH</string>
    </dict>
</dict>
</plist>
EOF

# Load and start the service
launchctl unload "$PLIST_PATH" 2>/dev/null || true  # Cleanup if already exists
launchctl load "$PLIST_PATH"
launchctl start com.neda

echo "✅ Neda service installed and started successfully!"
echo "Check logs: tail -f /tmp/neda.{log,err}"
echo "Uninstall later with: launchctl unload $PLIST_PATH && rm $PLIST_PATH"
