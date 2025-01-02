use crate::notification::{ComposeNotification, NotificationData};
use crate::{config::PowerConfig, modules::Module, utils::with_logs::WithLogs};

use udev::{EventType, MonitorBuilder, MonitorSocket};

pub struct PowerModule {
    monitor: MonitorSocket,
    config: PowerConfig,
}

impl Module for PowerModule {
    fn poll(&mut self) -> anyhow::Result<Option<notify_rust::Notification>> {
        let mut notification = None;

        if let Some(event) = self.monitor.iter().next() {
            let event_type = event.event_type();

            if event_type == EventType::Change
                && event.attribute_value("type").unwrap().to_str() == Some("Mains")
            {
                let is_charging =
                    matches!(event.attribute_value("online").unwrap().to_str(), Some("1"));

                if let Some(charging_config) = self.config.charging.as_ref() {
                    notification = charging_config.compose_notification(NotificationData::Power {
                        charging: if is_charging {
                            "Charging".to_string()
                        } else {
                            "Discharging".to_string()
                        },
                        percentage: 0,
                    });
                }
            }
        }

        Ok(notification)
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl PowerModule {
    pub const NAME: &str = "PowerModule";

    pub fn create(config: PowerConfig) -> anyhow::Result<Self> {
        (|| Self::initialize(config)).with_logs(Self::NAME, "Initialization")
    }

    fn initialize(config: PowerConfig) -> anyhow::Result<Self> {
        let builder =
            MonitorBuilder::new()?.match_subsystem_devtype("power_supply", "power_supply")?;

        let monitor: MonitorSocket = builder.listen()?;
        Ok(Self { monitor, config })
    }
}
