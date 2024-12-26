use std::time::Duration;

use anyhow::Ok;
use log::{debug, warn};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    config::{CommonConfig, Config},
    events::SystemEvent,
    modules::{
        device::{device::DeviceModule, notification::DeviceNotification},
        network::{network::NetworkModule, notification::NetworkNotification},
        power::{notification::PowerSupplyNotification, power_supply::PowerSupplyModule},
        Module,
    },
    notification::NotificationBuilder,
};

pub struct Hub {
    config: Config, // TODO: config
    modules: Vec<Box<dyn Module>>,
    sender: UnboundedSender<SystemEvent>,
    receiver: UnboundedReceiver<SystemEvent>,
}

impl Hub {
    pub fn init() -> anyhow::Result<Self> {
        let (sender, receiver) = unbounded_channel();
        let config = Self::load_cfg()?;

        let mut hub = Self {
            config,
            modules: Vec::new(),
            sender,
            receiver,
        };

        hub.setup()?;
        debug!(target: "Hub", "created");

        Ok(hub)
    }

    fn load_cfg() -> anyhow::Result<Config> {
        // TODO: config
        Ok(Config {
            network: CommonConfig { enabled: true },
            power_supply: CommonConfig { enabled: true },
            device: CommonConfig { enabled: true },
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.start_modules().await?;

        loop {
            tokio::select! {
                Some(event) = self.receiver.recv() => {
                    if let Err(e) = self.handle_event(event).await {
                        warn!("Failed to handle event: {}", e);
                    }
                }
            };

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn handle_event(&self, event: SystemEvent) -> anyhow::Result<()> {
        match event {
            SystemEvent::NetworkConnected {
                ssid,
                signal_strenght,
            } => NetworkNotification {
                ssid,
                signal_strenght,
            }
            .show()?,
            SystemEvent::PowerSupply { is_connected } => {
                PowerSupplyNotification { is_connected }.show()?
            }
            SystemEvent::Device { action, name } => DeviceNotification { action, name }.show()?,
        };

        Ok(())
    }

    async fn start_modules(&self) -> anyhow::Result<()> {
        for module in &self.modules {
            module.start()?;
        }

        Ok(())
    }

    fn setup(&mut self) -> anyhow::Result<()> {
        if self.config.network.enabled {
            self.register_module(Box::new(NetworkModule::new(self.sender.clone())));
        }
        if self.config.power_supply.enabled {
            self.register_module(Box::new(PowerSupplyModule::new(self.sender.clone())));
        }
        if self.config.device.enabled {
            self.register_module(Box::new(DeviceModule::new(self.sender.clone())));
        }

        self.init_modules()
    }

    fn register_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    fn init_modules(&mut self) -> anyhow::Result<()> {
        for module in &self.modules {
            module.init(self.sender.clone(), &self.config)?;
        }

        Ok(())
    }
}
