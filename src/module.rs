use tokio::sync::mpsc::UnboundedSender;

use crate::{config::Config, events::SystemEvent};

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn init(&self, sender: UnboundedSender<SystemEvent>, config: &Config) -> anyhow::Result<()>;
    fn start(&self) -> anyhow::Result<()>;
    fn configure(&mut self, config: &Config) -> anyhow::Result<()>;
}
