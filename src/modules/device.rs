use futures_util::TryStreamExt;
use log::{debug, error, trace};
use std::ffi::OsStr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

use crate::{config::Config, events::SystemEvent, module::Module, utils::with_logs::WithLogs};

pub struct DeviceModule {
    sender: UnboundedSender<SystemEvent>,
}

impl Module for DeviceModule {
    // type M = DeviceModule;

    fn init(&self, sender: UnboundedSender<SystemEvent>, config: &Config) -> anyhow::Result<()> {
        let mut module = DeviceModule { sender };
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
                    error!("Device module failed: {}", e);
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
        "DeviceModule"
    }
}

impl DeviceModule {
    pub fn new(sender: UnboundedSender<SystemEvent>) -> Self {
        Self { sender }
    }

    async fn run_monitor(sender: UnboundedSender<SystemEvent>) -> anyhow::Result<()> {
        let builder = MonitorBuilder::new()?.match_subsystem_devtype("usb", "usb_device")?;

        let monitor: AsyncMonitorSocket = builder.listen()?.try_into()?;
        Self::listen(monitor, sender).await
    }

    async fn listen(
        mut monitor: AsyncMonitorSocket,
        sender: UnboundedSender<SystemEvent>,
    ) -> anyhow::Result<()> {
        while let Some(event) = monitor.try_next().await? {
            let event_type = event.event_type();
            let subsystem = event
                .subsystem()
                .and_then(OsStr::to_str)
                .unwrap_or("unknown");
            let devtype = event.devtype().and_then(OsStr::to_str).unwrap_or("unknown");

            let name = Self::get_device_name(&event.device());

            trace!("Subsystem: {subsystem}, Devtype: {devtype}");

            sender.send(SystemEvent::Device {
                action: event_type,
                name,
            })?;
        }

        Ok(())
    }

    // TODO: format device name, e.g. DEVICE_NAME => Device Name or something
    fn get_device_name(device: &tokio_udev::Device) -> String {
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
