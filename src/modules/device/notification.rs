use notify_rust::Notification;
use tokio_udev::EventType;

use crate::notification::NotificationBuilder;

pub struct DeviceNotification {
    pub action: EventType,
    pub name: String,
}

impl NotificationBuilder for DeviceNotification {
    fn build(&self) -> anyhow::Result<Option<Notification>> {
        let action = match self.action {
            EventType::Add => "connected",
            EventType::Remove => "disconnected",
            _ => return Ok(None),
        };

        Ok(Some(
            Notification::new()
                .appname("device_module")
                .summary("Device")
                .body(format!("Device {}: {}", action, self.name).as_str())
                .finalize(),
        ))
    }
}
