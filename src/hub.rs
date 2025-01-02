use std::time::Duration;

use log::{debug, warn};

use crate::{config::Config, modules::Module};

pub struct Hub {
    config: Config, // TODO: config
    modules: Vec<Box<dyn Module>>,
}

impl Hub {
    pub fn init() -> anyhow::Result<Self> {
        let config = Self::load_cfg()?;

        let mut hub = Self {
            config,
            modules: Vec::new(),
        };

        hub.setup()?;
        debug!(target: "Hub", "created");

        Ok(hub)
    }

    fn load_cfg() -> anyhow::Result<Config> {
        // TODO: config
        Ok(Default::default())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            for module in self.modules.iter_mut() {
                if let Ok(Some(notification)) = module.poll() {
                    notification.show()?;
                }

                //TODO: error handling
                // if let Err(e) = self.handle_event(event).await {
                //     warn!("Failed to handle event: {}", e);
                // }
            }

            std::thread::sleep(Duration::from_millis(70));
        }
    }

    fn setup(&mut self) -> anyhow::Result<()> {
        macro_rules! register_modules {
            ($($module:ident),*) => {
                $(
                    match <Box<dyn Module>>::try_from(&self.config.$module) {
                        Ok(module) => self.register_module(module),
                        Err(err) => warn!("Failed to load module: {err}"),
                    }
                )*
            };
        }
        register_modules!(network, power, device);

        Ok(())
    }

    fn register_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }
}
