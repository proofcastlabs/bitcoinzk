//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use std::str::FromStr;
use sp1_zkvm::io::{write, read};
use derive_getters::Getters;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use ::hex::decode;
use bitcoin::{
    hex::HexToArrayError,
    blockdata::{
        block::{Version, Header as BtcBlockHeader, Block as BtcBlock},
        opcodes,
        script::{Builder as BtcScriptBuilder, Script as BtcScript},
        transaction::{OutPoint as BtcOutPoint, Transaction as BtcTransaction, TxIn as BtcUtxo, TxOut as BtcTxOut},
    },
    consensus::encode::deserialize as btc_deserialize,
    hash_types::{BlockHash, TxMerkleNode},
    CompactTarget,
    hashes::Hash,
};

#[derive(Debug, Error)]
enum Error {
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("hex array error: {0}")]
    HexArray(#[from] HexToArrayError),

    #[error("btc consensus encode error: {0}")]
    BtcConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[error("from hex error: {0}")]
    FromHex(#[from] ::hex::FromHexError),
}

struct BtcSubmissionMaterial {
    id: BlockHash,
    block: BtcBlock,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
struct BtcSubmissionMaterialJson {
    block: BtcBlockJson,
    transactions: Vec<String>,
}

impl FromStr for BtcSubmissionMaterialJson {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct BtcBlockJson {
    bits: u32,
    id: String,
    nonce: u32,
    version: i32,
    height: u64,
    timestamp: u32,
    merkle_root: String,
    previousblockhash: String,
}

impl FromStr for BtcSubmissionMaterial {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = BtcSubmissionMaterialJson::from_str(s)?;
        Ok(Self {
            id: BlockHash::from_str(j.block().id())
                .unwrap_or_else(|_| BlockHash::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap()),
            block: BtcBlock {
                txdata: j.transactions.iter().map(|t| Ok(btc_deserialize::<BtcTransaction>(&decode(t)?)?)).collect::<Result<Vec<_>, Self::Err>>()?,
                header: BtcBlockHeader {
                    nonce: *j.block().nonce(),
                    time: *j.block().timestamp(),
                    version: Version::from_consensus(*j.block().version()),
                    bits: CompactTarget::from_consensus(*j.block().bits()),
                    merkle_root: TxMerkleNode::from_str(&j.block().merkle_root)?,
                    prev_blockhash: BlockHash::from_str(&j.block().previousblockhash)?,
                },
            }
        })
    }
}

pub fn main() {
    let s = read::<String>();
    let b = BtcSubmissionMaterial::from_str(&s).expect("could not parse submission material");

    write(&true);
}
