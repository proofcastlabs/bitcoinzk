use crate::error::Error;

pub(crate) async fn run_timer(t: u64) -> Result<(), Error> {
    tokio::time::sleep(tokio::time::Duration::from_secs(t)).await;
    Ok(())
}
