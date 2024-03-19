use bitcoin::Block as BtcBlock;
use derive_more::{Constructor, Deref};

#[derive(Debug, Clone, Deref, Constructor)]
pub(crate) struct BtcBlocks(Vec<BtcBlock>);
