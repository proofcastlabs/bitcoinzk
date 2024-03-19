#!/bin/bash

# Exit early if non-zero exit code encountered
set -e

# Switch to dir this script lives in
cd $(dirname -- $0)

./create-elf.sh

RUST_LOG=debug cargo build --release -p bitcoinz-script
