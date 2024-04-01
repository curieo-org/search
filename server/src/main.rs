use server::{run, Result};

#[tokio::main]
async fn main() -> Result<()> {
    run().await?.run_until_stopped().await.map_err(Into::into)
}
