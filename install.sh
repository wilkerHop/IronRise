#!/bin/bash
set -e

APP_NAME="IronRise"
APP_DIR="/Applications/${APP_NAME}.app"
PLIST_NAME="com.ironrise.alarm.plist"
PLIST_SRC="scripts/${PLIST_NAME}"
PLIST_DEST="${HOME}/Library/LaunchAgents/${PLIST_NAME}"

echo "🚀 Starting Installation of ${APP_NAME}..."

# 1. Build the App
echo "📦 Building Application..."
npm run tauri build

# 2. Install .app
echo "📂 Installing to /Applications..."
# Tauri build output is usually in src-tauri/target/release/bundle/macos/
BUILD_DIR="src-tauri/target/release/bundle/macos"

if [ -d "${APP_DIR}" ]; then
    echo "  - Removing existing installation..."
    rm -rf "${APP_DIR}"
fi

cp -r "${BUILD_DIR}/${APP_NAME}.app" "/Applications/"

# 3. Install Launch Agent
echo "⚙️ Configuring Launch Agent..."
if [ ! -d "${HOME}/Library/LaunchAgents" ]; then
    mkdir -p "${HOME}/Library/LaunchAgents"
fi

cp "${PLIST_SRC}" "${PLIST_DEST}"

# 4. Load Agent
echo "🔄 Loading Launch Agent..."
# Unload first just in case
launchctl unload "${PLIST_DEST}" 2>/dev/null || true
launchctl load "${PLIST_DEST}"

echo "✅ Installation Complete!"
echo "   You can find ${APP_NAME} in /Applications."
