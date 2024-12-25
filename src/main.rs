use hub::Hub;

mod hub;
mod module;
mod modules;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();
    Hub::init()?.run().await
}

fn setup_logger() {
    const ENV_NAME: &str = "NOTIHUB_LOG";
    env_logger::Builder::from_env(env_logger::Env::default().filter_or(ENV_NAME, "info")).init();
}
