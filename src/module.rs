use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::{config::Config, events::SystemEvent};

pub trait Module: Send + Sync + Sized {
    type M: Module;

    fn name() -> &'static str;
    fn init(
        &self,
        sender: UnboundedSender<SystemEvent>,
        config: &Config,
    ) -> anyhow::Result<Self::M>;
    fn start(self: Arc<Self>) -> anyhow::Result<()>;
    fn configure(&mut self, config: &Config) -> anyhow::Result<()>;
}
