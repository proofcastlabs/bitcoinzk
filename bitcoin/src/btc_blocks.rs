use crate::{
    btc_block_json::{BtcBlockJson, BtcBlockJsons},
    BtcError,
};
use bitcoin::blockdata::block::Block as BtcBlock;
use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Deref, Constructor)]
pub struct BtcBlocks(Vec<BtcBlock>);

impl TryFrom<BtcBlockJsons> for BtcBlocks {
    type Error = BtcError;

    fn try_from(x: BtcBlockJsons) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl TryFrom<&BtcBlockJsons> for BtcBlocks {
    type Error = BtcError;

    fn try_from(x: &BtcBlockJsons) -> Result<Self, Self::Error> {
        Ok(Self::new(
            x.iter()
                .map(|y| y.to_btc_block())
                .collect::<Result<Vec<_>, Self::Error>>()?,
        ))
    }
}

impl fmt::Display for BtcBlocks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r: Result<BtcBlockJsons, BtcError> = self.try_into();
        match r {
            Ok(jsons) => write!(f, "{}", json!(jsons)),
            Err(e) => write!(f, "{e}"),
        }
    }
}

impl FromStr for BtcBlocks {
    type Err = BtcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let js: BtcBlockJsons = serde_json::from_str(&s)?;
        Self::try_from(js)
    }
}

impl TryFrom<Vec<u8>> for BtcBlocks {
    type Error = BtcError;

    fn try_from(bs: Vec<u8>) -> Result<Self, Self::Error> {
        let jsons: BtcBlockJsons = BtcBlockJsons::new(serde_json::from_slice(&bs)?);
        Self::try_from(jsons)
    }
}

impl TryInto<Vec<u8>> for BtcBlocks {
    type Error = BtcError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let jsons: BtcBlockJsons = self.try_into()?;
        Ok(serde_json::to_vec(&jsons)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::get_sample_btc_blocks_1;

    #[test]
    fn should_parse_sample_blocks_json_correctly() {
        let blocks = get_sample_btc_blocks_1();
        let s = blocks.to_string();
        let r = BtcBlocks::from_str(&s).unwrap();
        assert_eq!(blocks, r)
    }
}
