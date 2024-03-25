use server::run;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    run().await?.run_until_stopped().await
}
