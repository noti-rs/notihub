use hub::Hub;

mod config;
mod hub;
mod modules;
mod notification;
mod utils;

fn main() -> anyhow::Result<()> {
    setup_logger();
    Hub::init()?.run()
}

fn setup_logger() {
    const ENV_NAME: &str = "NOTI_LOG";
    env_logger::Builder::from_env(env_logger::Env::default().filter_or(ENV_NAME, "info")).init();
}
