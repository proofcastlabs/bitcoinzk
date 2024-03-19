use ::hex::decode;
use bitcoin::{
    blockdata::{
        block::{Block as BtcBlock, Header as BtcBlockHeader, Version},
        transaction::Transaction as BtcTransaction,
    },
    consensus::encode::deserialize as btc_deserialize,
    hash_types::{BlockHash, TxMerkleNode},
    hex::HexToArrayError,
    CompactTarget,
};
use derive_getters::Getters;
use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BtcSubmissionMaterialError {
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("hex array error: {0}")]
    HexArray(#[from] HexToArrayError),

    #[error("btc consensus encode error: {0}")]
    BtcConsensusEncode(#[from] bitcoin::consensus::encode::Error),

    #[error("from hex error: {0}")]
    FromHex(#[from] ::hex::FromHexError),
}

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, Deref)]
pub(crate) struct BtcSubmissionMaterials(Vec<BtcSubmissionMaterial>);

impl FromStr for BtcSubmissionMaterials {
    type Err = BtcSubmissionMaterialError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let js = serde_json::from_str::<Vec<BtcSubmissionMaterialJson>>(s)?;
        let r = js
            .iter()
            .map(BtcSubmissionMaterial::try_from)
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(Self::new(r))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Constructor)]
pub(crate) struct BtcSubmissionMaterial {
    id: BlockHash,
    block: BtcBlock,
}

impl BtcSubmissionMaterial {
    pub(crate) fn block_hash(&self) -> BlockHash {
        self.block().block_hash()
    }

    pub(crate) fn check_merkle_root(&self) -> bool {
        self.block().check_merkle_root()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
struct BtcSubmissionMaterialJson {
    block: BtcBlockJson,
    transactions: Vec<String>,
}

impl FromStr for BtcSubmissionMaterialJson {
    type Err = BtcSubmissionMaterialError;

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

impl TryFrom<BtcSubmissionMaterialJson> for BtcSubmissionMaterial {
    type Error = BtcSubmissionMaterialError;

    fn try_from(j: BtcSubmissionMaterialJson) -> Result<Self, Self::Error> {
        BtcSubmissionMaterial::try_from(&j)
    }
}

impl TryFrom<&BtcSubmissionMaterialJson> for BtcSubmissionMaterial {
    type Error = BtcSubmissionMaterialError;

    fn try_from(j: &BtcSubmissionMaterialJson) -> Result<Self, Self::Error> {
        Ok(Self {
            id: BlockHash::from_str(j.block().id()).unwrap_or_else(|_| {
                BlockHash::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap()
            }),
            block: BtcBlock {
                txdata: j
                    .transactions
                    .iter()
                    .map(|t| Ok(btc_deserialize::<BtcTransaction>(&decode(t)?)?))
                    .collect::<Result<Vec<_>, Self::Error>>()?,
                header: BtcBlockHeader {
                    nonce: *j.block().nonce(),
                    time: *j.block().timestamp(),
                    version: Version::from_consensus(*j.block().version()),
                    bits: CompactTarget::from_consensus(*j.block().bits()),
                    merkle_root: TxMerkleNode::from_str(&j.block().merkle_root)?,
                    prev_blockhash: BlockHash::from_str(&j.block().previousblockhash)?,
                },
            },
        })
    }
}

impl FromStr for BtcSubmissionMaterial {
    type Err = BtcSubmissionMaterialError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = BtcSubmissionMaterialJson::from_str(s)?;
        BtcSubmissionMaterial::try_from(j)
    }
}
