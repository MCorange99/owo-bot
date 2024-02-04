#!/usr/bin/bash

git pull origin main
cargo build --release
systemctl --user restart mcbot