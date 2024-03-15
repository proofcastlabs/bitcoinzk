//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

mod btc_submission_material;

use btc_submission_material::BtcSubmissionMaterials;
use sp1_zkvm::io::{read, write};
use std::str::FromStr;

fn main() {
    let s = read::<String>();
    let sub_mats =
        BtcSubmissionMaterials::from_str(&s).expect("could not parse submission material");
    let sub_mat = sub_mats[0].clone();
    let id = sub_mat.id();
    let hash = sub_mat.block_hash();
    println!("  hash in header: {id}");
    println!(" calculated hash: {hash}");
    let header_hash_result = &hash == id;
    let tx_merkle_root_result = sub_mat.check_merkle_root();
    write(&(header_hash_result && tx_merkle_root_result));
}
