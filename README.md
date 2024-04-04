# sp1

__:radioactive: IMPORTANT:__ Read the `./sp1/program/README.md` first to make sure you can successfully build the `elf`.

Then, to build everything, in the root just run the build script:

`./build-sp1.sh`

This will first create the `elf` and then build the CLI.

Then, for usage info, run the binary which can be found in:

`./target/release/bitcoinz-sp1-cli --help`

Output:

```
Usage: bitcoinz-sp1-cli <COMMAND>

Commands:
  generateProof  Generate proof
  getBlocks      Get BTC blocks for ZKP light-client proof generation
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

__Example:__

After building, get a BTC rpc endpoint and use the CLI to download a batch of blocks. (Currently limited to 10). Then, once downloaded, use the CLI to generate a proof passing the files those blocks were saved to as an argument.

```
./target/release/bitcoinz-sp1-cli getBlocks <btc-endpoint> <startBlockNum> <numBlocks> --output=<blocksFileName>

&&

RUST_LOG=debug ./target/release/bitcoinz-sp1-cli generateProof <blocksFileName>

```
