#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::io::{read, write};

use lc::prove_btc_blocks_from_string;

fn main() {
    let blocks = read::<String>();

    write(&prove_btc_blocks_from_string(blocks))
}
