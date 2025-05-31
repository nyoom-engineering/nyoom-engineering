#!/usr/bin/env bash
# projectgen.sh — generate themed icons, render them, add shadows
#                 (macOS & Linux, no GNU‑only tools)
# Usage: ./projectgen.sh "<app name>" "<theme name>"

set -euo pipefail

if [[ $# -ne 2 ]]; then
  printf 'Usage: %s <app-name> <theme-name>\n' "$0" >&2
  exit 1
fi

APP="$1"
THEME="$2"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_REPLACE="$SCRIPT_DIR/svg-replace/target/release/svg_replace"
BIN_RENDER="$SCRIPT_DIR/svg-renderer/target/release/svg_renderer"

for bin in "$BIN_REPLACE" "$BIN_RENDER"; do
  [[ -x $bin ]] || { printf 'Error: binary not found or not executable → %s\n' "$bin" >&2; exit 1; }
done

printf 'Generating %s …\n'
"$BIN_REPLACE" "$APP" "$THEME"

printf 'Rendering PNG(s) …\n'
"$BIN_RENDER" "logo-custom.svg"

printf 'Adding shadows …\n'
find . -type f -name '*.png' ! -name '*-shadow.png' -exec \
  sh -c '
    for img; do
      magick "$img" \
        \( +clone -background black -shadow 40x50+0+36 \) \
        +swap -background transparent -layers merge +repage \
        "${img%.png}-shadow.png"
    done
  ' _ {} +

printf 'done'