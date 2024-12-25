use std::sync::Arc;

use log::debug;
use tokio::sync::mpsc::UnboundedSender;

use crate::{hub, hub::Config, module::Module, utils::with_logs::WithLogs};

pub struct PowerModule {
    sender: UnboundedSender<hub::SystemEvent>,
}

impl PowerModule {}

// TODO: ...
impl Module for PowerModule {
    type M = PowerModule;

    fn init(
        &self,
        sender: UnboundedSender<hub::SystemEvent>,
        config: &Config,
    ) -> anyhow::Result<Self::M> {
        let mut module = PowerModule { sender };
        module.with_logs(Self::name(), "initializing", |m| m.configure(config))?;

        Ok(module)
    }

    fn configure(&mut self, config: &Config) -> anyhow::Result<()> {
        self.with_logs(Self::name(), "configuring", |_| {});

        Ok(())
    }

    fn start(self: Arc<Self>) -> anyhow::Result<()> {
        let self_clone = self.clone();

        std::thread::spawn(|| {
            debug!(target: "SystemHub", "starting {} module", Self::name());
            // self_clone.listen();
        });

        Ok(())
    }

    fn name() -> &'static str {
        "PowerModule"
    }
}
