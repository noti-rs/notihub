use std::sync::Arc;

use log::{debug, error};
use notify_rust::Notification;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    module::Module,
    modules::{network::NetworkModule, power::PowerModule},
};

pub struct Config {
    pub network: NetworkConfig,
}

pub struct NetworkConfig {
    pub enabled: bool,
}

pub struct Hub {
    config: Config, // TODO: config
    modules: Vec<HubModule>,
    sender: UnboundedSender<SystemEvent>,
    receiver: UnboundedReceiver<SystemEvent>,
}

enum HubModule {
    Network(Arc<NetworkModule>),
    Power(Arc<PowerModule>),
}

pub enum SystemEvent {
    NetworkConnected { ssid: String },
    PowerLowBattery { level: u8 },
    PowerCharging,
    DeviceAdded { device_name: String },
}

impl Hub {
    pub fn init() -> anyhow::Result<Self> {
        let (sender, receiver) = unbounded_channel();
        let modules = vec![];

        // TODO: config
        let config = Config {
            network: NetworkConfig { enabled: true },
        };

        let mut hub = Self {
            config,
            modules,
            sender,
            receiver,
        };

        hub.setup()?;
        debug!(target: "Hub", "created");

        Ok(hub)
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.start_modules().await?;

        loop {
            tokio::select! {
                Some(event) = self.receiver.recv() => {
                    if let Err(e) = self.handle_event(event).await {
                        error!("Failed to handle event: {}", e);
                    }
                }
            };
        }
    }

    async fn start_modules(&self) -> anyhow::Result<()> {
        for module in &self.modules {
            match module {
                HubModule::Network(module) => {
                    module.clone().start()?;
                }
                HubModule::Power(module) => {
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
                HubModule::Power(module) => {
                    module.init(self.sender.clone(), &self.config)?;
                }
            }
        }

        Ok(())
    }

    async fn handle_event(&self, event: SystemEvent) -> anyhow::Result<()> {
        match event {
            SystemEvent::NetworkConnected { ssid } => Notification::new()
                .appname("network_module")
                .image_path("/path/to/icon.svg")
                .summary("Network")
                .body(format!("Connected to {}", ssid).as_str())
                .show()?,
            _ => unimplemented!(),
        };

        Ok(())
    }
}
