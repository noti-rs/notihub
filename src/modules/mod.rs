pub mod device;
pub mod network;
pub mod power;

use crate::config::{DeviceConfig, NetworkConfig, PowerConfig};

use device::DeviceModule;
use network::NetworkModule;
use notify_rust::Notification;
use power::PowerModule;

pub trait Module {
    fn name(&self) -> &'static str;
    fn poll(&mut self) -> anyhow::Result<Option<Notification>>;
}

impl TryFrom<&NetworkConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    // TODO: return info about disabled submodules

    fn try_from(value: &NetworkConfig) -> Result<Self, Self::Error> {
        Ok(NetworkModule::create(value.clone()).map(Box::new)?)
    }
}

impl TryFrom<&PowerConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    // TODO: return info about disabled submodules

    fn try_from(value: &PowerConfig) -> Result<Self, Self::Error> {
        Ok(PowerModule::create(value.clone()).map(Box::new)?)
    }
}

impl TryFrom<&DeviceConfig> for Box<dyn Module> {
    type Error = anyhow::Error;

    // TODO: return info about disabled submodules

    fn try_from(value: &DeviceConfig) -> Result<Self, Self::Error> {
        Ok(DeviceModule::create(value.clone()).map(Box::new)?)
    }
}
