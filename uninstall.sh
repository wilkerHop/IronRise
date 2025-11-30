#!/bin/bash
set -e

APP_NAME="IronRise"
APP_DIR="/Applications/${APP_NAME}.app"
PLIST_NAME="com.ironrise.alarm.plist"
PLIST_DEST="${HOME}/Library/LaunchAgents/${PLIST_NAME}"

echo "🗑️  Uninstalling ${APP_NAME}..."

# 1. Unload Launch Agent
if launchctl list | grep -q "com.ironrise.alarm"; then
    echo "  - Unloading Launch Agent..."
    launchctl unload "${PLIST_DEST}" 2>/dev/null || true
fi

# 2. Remove Launch Agent File
if [ -f "${PLIST_DEST}" ]; then
    echo "  - Removing plist file..."
    rm "${PLIST_DEST}"
fi

# 3. Remove Application
if [ -d "${APP_DIR}" ]; then
    echo "  - Removing Application..."
    rm -rf "${APP_DIR}"
fi

# 4. Optional: Clear pmset schedule
if [ "$1" == "--no-prompt" ] || [ -n "$CI" ]; then
    echo "⏩ Skipping pmset clear (CI/Non-interactive mode)."
else
    echo "❓ Do you want to clear any scheduled wake events? (Requires Admin Password)"
    read -p "   Run 'sudo pmset repeat cancel'? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        sudo pmset repeat cancel
        echo "   Schedule cleared."
    fi
fi

echo "✅ Uninstallation Complete."
