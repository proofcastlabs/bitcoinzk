use crate::{
    btc_block_json::BtcBlockJson, btc_blocks::BtcBlocks, curl, get_block_hashes::get_block_hashes,
    json_response::JsonResponse, BtcError,
};
use bitcoin::{blockdata::block::Block as BtcBlock, BlockHash};
use futures::{stream, Future, Stream, StreamExt};
use serde_json::json;
use std::{
    process::Output,
    str::{from_utf8, FromStr},
};

const MAX_CONCURRENT_REQUESTS: usize = 50;
const VERBOSITY: u64 = 2; // NOTE: This gets tx data too: https://developer.bitcoin.org/reference/rpc/getblock.html
const RPC_METHOD: &str = "getblock";

async fn get_block_future(
    rpc_endpoint: &str,
    block_hash: BlockHash,
) -> impl Future<Output = Result<Output, BtcError>> + '_ {
    curl(rpc_endpoint, RPC_METHOD, json!([block_hash, VERBOSITY]))
}

fn get_get_block_futures(
    rpc_endpoint: &str,
    block_hashes: Vec<BlockHash>,
) -> impl Stream<Item = impl Future<Output = Result<Output, BtcError>> + '_> {
    stream::iter(block_hashes).then(|block_hash| get_block_future(rpc_endpoint, block_hash))
}

pub async fn get_blocks(
    rpc_endpoint: &str,
    start: u64,
    amount: u64,
) -> Result<BtcBlocks, BtcError> {
    let block_hashes = get_block_hashes(rpc_endpoint, start, amount).await?;
    let futures = get_get_block_futures(rpc_endpoint, block_hashes);

    let responses = futures
        .buffered(MAX_CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await;

    Ok(BtcBlocks::new(
        responses
            .into_iter()
            .map(|r| serde_json::from_str::<JsonResponse>(from_utf8(&r?.stdout)?)?.result())
            .map(|r| {
                // TODO FIXME Handle the case where r is_err/none because there's no block
                let block_json = BtcBlockJson::from_str(&r?.to_string())?; // FIXME very inefficient
                block_json.to_btc_block()
            })
            .collect::<Result<Vec<BtcBlock>, BtcError>>()?,
    ))
}
