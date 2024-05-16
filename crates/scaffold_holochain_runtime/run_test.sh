#!/usr/bin/bash
set -e

rm -rf /tmp/test-scaffold-holochain-runtime

nix run .#scaffold-holochain-runtime -- test-scaffold-holochain-runtime --path /tmp
cd /tmp/test-scaffold-holochain-runtime

nix flake update
nix develop --command bash -c "
set -e
npm i
npm run tauri build
"
