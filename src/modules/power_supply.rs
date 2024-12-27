use notify_rust::Notification;
use udev::{EventType, MonitorBuilder, MonitorSocket};

use crate::{
    config::PSConfig,
    modules::Module,
    utils::{make_notification::MakeNotification, with_logs::WithLogs},
};

pub struct PowerSupplyModule {
    monitor: MonitorSocket,
}

impl Module for PowerSupplyModule {
    fn poll(&mut self) -> anyhow::Result<Option<notify_rust::Notification>> {
        let mut notification = None;

        if let Some(event) = self.monitor.iter().next() {
            let event_type = event.event_type();

            if event_type == EventType::Change
                && event.attribute_value("type").unwrap().to_str() == Some("Mains")
            {
                let is_connected =
                    matches!(event.attribute_value("online").unwrap().to_str(), Some("1"));

                notification = is_connected.make_notification();
            }
        }

        Ok(notification)
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl PowerSupplyModule {
    const NAME: &str = "PowerSupplyModule";

    pub fn create(config: &PSConfig) -> anyhow::Result<Self> {
        (|| Self::initialize(config)).with_logs(Self::NAME, "Initialization")
    }

    fn initialize(_config: &PSConfig) -> anyhow::Result<Self> {
        let builder =
            MonitorBuilder::new()?.match_subsystem_devtype("power_supply", "power_supply")?;

        let monitor: MonitorSocket = builder.listen()?;
        Ok(Self { monitor })
    }
}

impl MakeNotification for bool {
    fn make_notification(self) -> Option<notify_rust::Notification> {
        let is_connected = self;
        let status = if is_connected {
            "Charging"
        } else {
            "Discharging"
        };

        Some(
            Notification::new()
                .appname("power_supply_module")
                .summary("Power")
                .body(status)
                .finalize(),
        )
    }
}
