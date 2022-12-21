mod cli;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::engine::initialize().await
}
