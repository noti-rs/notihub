use notify_rust::Notification;

use crate::notification::NotificationBuilder;

pub struct NetworkNotification {
    pub ssid: String,
    pub signal_strenght: u8,
}

impl NotificationBuilder for NetworkNotification {
    fn build(&self) -> anyhow::Result<Option<Notification>> {
        Ok(Some(
            Notification::new()
                .appname("network_module")
                .summary("Network")
                .body(format!("Connected to {}", self.ssid).as_str())
                .finalize(),
        ))
    }
}
