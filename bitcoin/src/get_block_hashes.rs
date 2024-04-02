use crate::{curl, BtcError, JsonResponse};
use bitcoin::{hashes::Hash, BlockHash};
use futures::{stream, Future, Stream, StreamExt};
use serde_json::json;
use std::{process::Output, str::from_utf8};

const MAX_CONCURRENT_REQUESTS: usize = 50;

async fn get_block_hash_future(
    rpc_endpoint: &str,
    n: u64,
) -> impl Future<Output = Result<Output, BtcError>> + '_ {
    curl(rpc_endpoint, "getblockhash", json!([n]))
}

fn get_get_block_hash_futures<'a>(
    rpc_endpoint: &'a str,
    block_nums: &'a [u64],
) -> impl Stream<Item = impl Future<Output = Result<Output, BtcError>> + 'a> + 'a {
    stream::iter(block_nums).then(|n| get_block_hash_future(rpc_endpoint, *n))
}

pub(crate) async fn get_block_hashes(
    rpc_endpoint: &str,
    start: u64,
    amount: u64,
) -> Result<Vec<BlockHash>, BtcError> {
    let block_nums = (0..amount).map(|i| start + i).collect::<Vec<u64>>();
    let futures = get_get_block_hash_futures(rpc_endpoint, &block_nums);

    let responses = futures
        .buffered(MAX_CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await;

    let json_responses = responses
        .into_iter()
        .map(|r| {
            Ok(serde_json::from_str::<JsonResponse>(from_utf8(
                &r?.stdout,
            )?)?)
        })
        .collect::<Result<Vec<JsonResponse>, BtcError>>()?;

    json_responses
        .into_iter()
        .map(|j| j.result())
        .map(|s| {
            let mut bs = hex::decode(s?.to_string().replace('"', ""))?;
            bs.reverse();
            Ok(BlockHash::from_slice(&bs)?)
        })
        .collect::<Result<Vec<_>, BtcError>>()
}
