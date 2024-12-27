use log::trace;
use notify_rust::Notification;
use std::ffi::OsStr;
use udev::{EventType, MonitorBuilder, MonitorSocket};

use crate::{
    config::DeviceConfig,
    modules::Module,
    utils::{make_notification::MakeNotification, with_logs::WithLogs},
};

pub struct DeviceModule {
    monitor: MonitorSocket,
}

impl Module for DeviceModule {
    fn poll(&mut self) -> anyhow::Result<Option<notify_rust::Notification>> {
        let mut notification = None;

        if let Some(event) = self.monitor.iter().next() {
            let event_type = event.event_type();
            let subsystem = event
                .subsystem()
                .and_then(OsStr::to_str)
                .unwrap_or("unknown");
            let devtype = event.devtype().and_then(OsStr::to_str).unwrap_or("unknown");

            let name = Self::get_device_name(&event.device());

            trace!("Subsystem: {subsystem}, Devtype: {devtype}");

            notification = (event_type, name).make_notification();
        }

        Ok(notification)
    }

    fn name(&self) -> &'static str {
        "DeviceModule"
    }
}

impl DeviceModule {
    const NAME: &str = "DeviceModule";

    pub fn create(config: &DeviceConfig) -> anyhow::Result<Self> {
        (|| Self::initialize(config)).with_logs(Self::NAME, "Initialization")
    }

    fn initialize(_config: &DeviceConfig) -> anyhow::Result<Self> {
        let builder = MonitorBuilder::new()?.match_subsystem_devtype("usb", "usb_device")?;

        let monitor: MonitorSocket = builder.listen()?;
        Ok(Self { monitor })
    }

    // TODO: format device name, e.g. DEVICE_NAME => Device Name or something
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

impl MakeNotification for (udev::EventType, String) {
    fn make_notification(self) -> Option<Notification> {
        let (event_type, name) = self;
        let action = match event_type {
            EventType::Add => "connected",
            EventType::Remove => "disconnected",
            _ => return None,
        };

        Some(
            Notification::new()
                .appname("device_module")
                .summary("Device")
                .body(format!("Device {}: {}", action, name).as_str())
                .finalize(),
        )
    }
}
