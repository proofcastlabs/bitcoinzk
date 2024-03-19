use crate::BtcError;
use serde_json::{json, Value as Json};
use std::process::{Command, Output};

pub(crate) async fn curl(
    rpc_endpoint: &str,
    rpc_method: &str,
    params: Json,
) -> Result<Output, BtcError> {
    let request = json!({
        "id": "1",
        "jsonrpc": "1.0",
        "params": params,
        "method": rpc_method,
    });

    Ok(Command::new("sh")
        .arg("-c")
        .arg(format!("curl --data-binary '{}' {}", request, rpc_endpoint))
        .output()?)
}
