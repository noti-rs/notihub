use anyhow::bail;
use device::DeviceModule;
use network::NetworkModule;
use notify_rust::Notification;
use power_supply::PowerSupplyModule;

use crate::config::{DeviceConfig, NetworkConfig, PSConfig};

pub mod device;
pub mod network;
pub mod power_supply;

pub trait Module {
    fn name(&self) -> &'static str;
    fn poll(&mut self) -> anyhow::Result<Option<Notification>>;
}

impl TryFrom<&NetworkConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    fn try_from(value: &NetworkConfig) -> Result<Self, Self::Error> {
        if !value.enabled {
            bail!("Network disabled");
        }

        Ok(NetworkModule::create(value).map(Box::new)?)
    }
}

impl TryFrom<&PSConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    fn try_from(value: &PSConfig) -> Result<Self, Self::Error> {
        if !value.enabled {
            bail!("Power Supply disabled");
        }

        Ok(PowerSupplyModule::create(value).map(Box::new)?)
    }
}

impl TryFrom<&DeviceConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    fn try_from(value: &DeviceConfig) -> Result<Self, Self::Error> {
        if !value.enabled {
            bail!("Device disabled");
        }

        Ok(DeviceModule::create(value).map(Box::new)?)
    }
}
