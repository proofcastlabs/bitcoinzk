#!/bin/bash

# Exit early if non-zero exit code encountered
set -e

# Switch to dir this script lives in
cd $(dirname -- $0)

# check the expected env var is set

if [ -z "${RISCV}" ]; then
    echo "RISCV env var is not set! See README for details"
    exit 1
fi

cd ./program

CC_riscv32im_succinct_zkvm_elf=$RISCV cargo prove build

cd -

echo "elf created successfully in ./elf/"
