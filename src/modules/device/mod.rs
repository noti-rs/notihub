use crate::notification::{ComposeNotification, NotificationData};
use crate::{config::DeviceConfig, modules::Module, utils::with_logs::WithLogs};

use std::ffi::OsStr;
use udev::{MonitorBuilder, MonitorSocket};

pub struct DeviceModule {
    monitor: MonitorSocket,
    config: DeviceConfig,
}

impl Module for DeviceModule {
    fn poll(&mut self) -> anyhow::Result<Option<notify_rust::Notification>> {
        let mut notification = None;

        if let Some(event) = self.monitor.iter().next() {
            let _event_type = event.event_type();
            let _subsystem = event
                .subsystem()
                .and_then(OsStr::to_str)
                .unwrap_or("unknown");
            let _devtype = event.devtype().and_then(OsStr::to_str).unwrap_or("unknown");

            let name = Self::get_device_name(&event.device());

            if let Some(charging_config) = self.config.connected.as_ref() {
                notification =
                    charging_config.compose_notification(NotificationData::Device { name });
            }
        }

        Ok(notification)
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl DeviceModule {
    pub const NAME: &'static str = "DeviceModule";

    pub fn create(config: DeviceConfig) -> anyhow::Result<Self> {
        (|| Self::initialize(config)).with_logs(Self::NAME, "Initialization")
    }

    fn initialize(config: DeviceConfig) -> anyhow::Result<Self> {
        let builder = MonitorBuilder::new()?.match_subsystem_devtype("usb", "usb_device")?;

        let monitor: MonitorSocket = builder.listen()?;
        Ok(Self { monitor, config })
    }

    // TODO: format device name, e.g. "DEVICE_NAME" => "Device Name" or something
    fn get_device_name(device: &udev::Device) -> String {
        let model = device
            .property_value("ID_MODEL")
            .map(|v| v.to_string_lossy().into_owned());
        let vendor = device
            .property_value("ID_VENDOR")
            .map(|v| v.to_string_lossy().into_owned());

        match (vendor, model) {
            (Some(vendor), Some(model)) => format!("{} {}", vendor, model),
            (Some(vendor), None) => vendor,
            (None, Some(model)) => model,
            _ => String::from("Unknown Device"),
        }
    }
}
