#!/bin/bash

# === CONFIGURATION ===

# SSH Infos
DISTANT_USER=remy
DISTANT_HOST=51.38.63.145
DOSSIER_DISTANT=/opt/dev/bozos

# Local destination folder
DOSSIER_LOCAL=~/Documents/dev/bozos/

# ======================

echo "📂 Syncing project from remote (excluding .gitignore rules)..."
rsync -avz \
  --exclude-from=<(ssh "$DISTANT_USER@$DISTANT_HOST" "cat '$DOSSIER_DISTANT/.gitignore'") \
  "$DISTANT_USER@$DISTANT_HOST:$DOSSIER_DISTANT/" \
  "$DOSSIER_LOCAL/"

echo "✅ Project successfully synced into $DOSSIER_LOCAL"
cd "$DOSSIER_LOCAL" || exit 1
cargo run
