use crate::{btc_blocks::BtcBlocks, Error};
use bitcoin::blockdata::block::Block as BtcBlock;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize, Deref)]
pub(crate) struct BtcBlocksWrapper(Vec<BtcBlock>);

impl BtcBlocksWrapper {
    pub(crate) fn new(bs: BtcBlocks) -> Self {
        Self(bs.iter().cloned().collect::<Vec<_>>())
    }
}

impl fmt::Display for BtcBlocksWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self))
    }
}

impl FromStr for BtcBlocksWrapper {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl TryFrom<Vec<u8>> for BtcBlocksWrapper {
    type Error = Error;

    fn try_from(bs: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&bs)?)
    }
}

impl TryInto<Vec<u8>> for BtcBlocksWrapper {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_vec(&self)?)
    }
}
