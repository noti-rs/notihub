use futures_util::TryStreamExt;
use log::{debug, error};
use tokio::sync::mpsc::UnboundedSender;
use tokio_udev::{AsyncMonitorSocket, EventType, MonitorBuilder};

use crate::{config::Config, events::SystemEvent, modules::Module, utils::with_logs::WithLogs};

pub struct PowerSupplyModule {
    sender: UnboundedSender<SystemEvent>,
}

impl Module for PowerSupplyModule {
    fn init(&self, sender: UnboundedSender<SystemEvent>, config: &Config) -> anyhow::Result<()> {
        let mut module = PowerSupplyModule { sender };
        module.with_logs(self.name(), "initializing", |m| m.configure(config))?;
        Ok(())
    }

    fn start(&self) -> anyhow::Result<()> {
        let sender = self.sender.clone();

        debug!(target: "Hub", "starting {} module", self.name());
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                if let Err(e) = Self::run_monitor(sender).await {
                    error!("PowerSupply module failed: {}", e);
                }
            });
        });

        Ok(())
    }

    fn configure(&mut self, _config: &Config) -> anyhow::Result<()> {
        self.with_logs(self.name(), "configuring", |_| {});
        Ok(())
    }

    fn name(&self) -> &'static str {
        "PowerSupplyModule"
    }
}

impl PowerSupplyModule {
    pub fn new(sender: UnboundedSender<SystemEvent>) -> Self {
        Self { sender }
    }

    async fn run_monitor(sender: UnboundedSender<SystemEvent>) -> anyhow::Result<()> {
        let builder =
            MonitorBuilder::new()?.match_subsystem_devtype("power_supply", "power_supply")?;

        let monitor: AsyncMonitorSocket = builder.listen()?.try_into()?;
        Self::listen(monitor, sender).await
    }

    async fn listen(
        mut monitor: AsyncMonitorSocket,
        sender: UnboundedSender<SystemEvent>,
    ) -> anyhow::Result<()> {
        while let Some(event) = monitor.try_next().await? {
            let event_type = event.event_type();

            if event_type == EventType::Change
                && event.attribute_value("type").unwrap().to_str() == Some("Mains")
            {
                let is_connected =
                    matches!(event.attribute_value("online").unwrap().to_str(), Some("1"));

                sender.send(SystemEvent::PowerSupply { is_connected })?;
            }
        }

        Ok(())
    }
}