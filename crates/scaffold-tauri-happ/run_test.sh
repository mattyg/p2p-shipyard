#!/usr/bin/bash
set -e

DIR=$(pwd)

nix shell --refresh github:holochain/scaffolding/develop-0.3#hc-scaffold --command bash -c "
cd /tmp
rm -rf forum-scaffold-tauri-happ

hc-scaffold --template lit web-app forum-scaffold-tauri-happ --setup-nix true -F --package-manager npm
cd /tmp/forum-scaffold-tauri-happ
nix flake update
nix develop --command bash -c \"hc-scaffold --version && npm i && hc scaffold dna forum && hc scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/\"
"

nix run --no-update-lock-file --accept-flake-config .#scaffold-tauri-happ -- --path /tmp/forum-scaffold-tauri-happ --ui-package ui --bundle-identifier org.myorg.myapp

cd /tmp/forum-scaffold-tauri-happ

nix develop --no-update-lock-file --override-input p2p-shipyard $DIR --command bash -c "
set -e

npm install
npm run tauri icon $DIR/examples/end-user-happ/src-tauri/icons/icon.png
cd src-tauri
cargo add -p forum-scaffold-tauri-happ --path $DIR/crates/tauri-plugin-holochain
cd ..
npm run build:happ
npm run tauri build -- --no-bundle
"

nix develop --no-update-lock-file --override-input p2p-shipyard $DIR .#androidDev --command bash -c "
set -e

npm install
npm run tauri android init
npm run tauri android build -- --target aarch64
"
