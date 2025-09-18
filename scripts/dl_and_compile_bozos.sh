#!/bin/bash

# === CONFIGURATION ===

# SSH Infos
DISTANT_USER=remy
DISTANT_HOST=51.38.63.145
DOSSIER_DISTANT=/opt/dev/bozos

# Local destination folder
DOSSIER_LOCAL=~/Documents/dev/bozos/

# ======================

# Temporary archive
TMP_TAR=/tmp/pfc_remote.tar.gz

echo "📦 Creating archive on remote machine..."
ssh "$DISTANT_USER@$DISTANT_HOST" "
  cd '$DOSSIER_DISTANT' &&
  tar czf /tmp/pfc_remote.tar.gz Cargo.toml src/ assets/ scripts/ README.md
"

echo "📥 Downloading archive..."
scp "$DISTANT_USER@$DISTANT_HOST:/tmp/pfc_remote.tar.gz" "$TMP_TAR"

echo "🧹 Local cleanup: removing src/ and Cargo.toml if present..."
rm -rf "$DOSSIER_LOCAL/src" "$DOSSIER_LOCAL/Cargo.toml" "$DOSSIER_LOCAL/assets" "$DOSSIER_LOCAL/README.md" "$DOSSIER_LOCAL/scripts"

echo "📂 Extracting into $DOSSIER_LOCAL..."
mkdir -p "$DOSSIER_LOCAL"
tar xzf "$TMP_TAR" -C "$DOSSIER_LOCAL"

echo "🧼 Cleaning up temporary files..."
rm "$TMP_TAR"
ssh "$DISTANT_USER@$DISTANT_HOST" "rm /tmp/pfc_remote.tar.gz"

echo "✅ Project successfully fetched into $DOSSIER_LOCAL"
cd "$DOSSIER_LOCAL"
cargo run
