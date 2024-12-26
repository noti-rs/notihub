use std::sync::Arc;

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
    modules: Vec<HubModule>,
    sender: UnboundedSender<SystemEvent>,
    notifier: Notifier,
}

enum HubModule {
    Network(Arc<NetworkModule>),
    PowerSupply(Arc<PowerSupplyModule>),
    Device(Arc<DeviceModule>),
}

impl Hub {
    pub fn init() -> anyhow::Result<Self> {
        let modules = vec![];

        let config = Self::load_cfg()?;

        let (notifier, sender) = Notifier::init()?;

        let mut hub = Self {
            config,
            modules,
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
            match module {
                HubModule::Network(module) => {
                    module.clone().start()?;
                }
                HubModule::PowerSupply(module) => {
                    module.clone().start()?;
                }
                HubModule::Device(module) => {
                    module.clone().start()?;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn setup(&mut self) -> anyhow::Result<()> {
        if self.config.network.enabled {
            let module = NetworkModule::new(self.sender.clone());
            self.register_module(HubModule::Network(Arc::new(module)));
        }

        if self.config.power_supply.enabled {
            let module = PowerSupplyModule::new(self.sender.clone());
            self.register_module(HubModule::PowerSupply(Arc::new(module)));
        }

        if self.config.device.enabled {
            let module = DeviceModule::new(self.sender.clone());
            self.register_module(HubModule::Device(Arc::new(module)));
        }

        // TODO: other modules

        self.init_modules()
    }

    fn register_module(&mut self, module: HubModule) {
        self.modules.push(module);
    }

    fn init_modules(&mut self) -> anyhow::Result<()> {
        for module in &self.modules {
            match module {
                HubModule::Network(module) => {
                    module.init(self.sender.clone(), &self.config)?;
                }
                HubModule::PowerSupply(module) => {
                    module.init(self.sender.clone(), &self.config)?;
                }
                HubModule::Device(module) => {
                    module.init(self.sender.clone(), &self.config)?;
                }
            }
        }

        Ok(())
    }
}
