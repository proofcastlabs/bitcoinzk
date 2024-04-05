Install the necessary dependencies for [risc0](https://dev.risczero.com/api/zkvm/install)

To create the elf, you'll need a prebuilt riscv compiler from [here](https://github.com/stnolting/riscv-gcc-prebuilt) (the `rv32i-131023` one).

Point to this precompile via the $RISCV env var.

Then just run the script `./build-risc0.sh`
