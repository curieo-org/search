use sentry::{self, ClientOptions};
use server::{run, Result};

fn main() -> Result<()> {
    let _ = sentry::init(ClientOptions {
        release: sentry::release_name!(),
        traces_sample_rate: 1.0,
        ..Default::default()
    });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await?.run_until_stopped().await.map_err(Into::into) })
}
