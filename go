#!/usr/bin/env bash
set -ex
cargo cbt -q
cargo run -q toml_edit
cargo run -q iocore
