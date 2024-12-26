use notify_rust::Notification;

use crate::notification::NotificationBuilder;

pub struct PowerSupplyNotification {
    pub is_connected: bool,
}

impl NotificationBuilder for PowerSupplyNotification {
    fn build(&self) -> anyhow::Result<Option<Notification>> {
        let status = if self.is_connected {
            "Charging"
        } else {
            "Discharging"
        };

        Ok(Some(
            Notification::new()
                .appname("power_supply_module")
                .summary("Power")
                .body(status)
                .finalize(),
        ))
    }
}
