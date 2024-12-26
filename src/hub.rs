use anyhow::Ok;
use log::debug;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    config::{CommonConfig, Config},
    events::SystemEvent,
    module::Module,
    modules::{device::DeviceModule, network::NetworkModule, power_supply::PowerSupplyModule},
    notifier::Notifier,
};

pub struct Hub {
    config: Config, // TODO: config
    modules: Vec<Box<dyn Module>>,
    sender: UnboundedSender<SystemEvent>,
    notifier: Notifier,
}

impl Hub {
    pub fn init() -> anyhow::Result<Self> {
        let config = Self::load_cfg()?;

        let (notifier, sender) = Notifier::init()?;

        let mut hub = Self {
            config,
            modules: Vec::new(),
            sender,
            notifier,
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

        self.notifier.run().await
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
