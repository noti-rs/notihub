use std::time::Duration;

use anyhow::bail;
use log::warn;
use notify_rust::Notification;
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::sleep,
};
use tokio_udev::EventType;

use crate::events::SystemEvent;

pub struct Notifier {
    receiver: UnboundedReceiver<SystemEvent>,
}

impl Notifier {
    pub fn init() -> anyhow::Result<(Self, UnboundedSender<SystemEvent>)> {
        let (sender, receiver) = unbounded_channel();

        let notifier = Notifier { receiver };

        Ok((notifier, sender))
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                Some(event) = self.receiver.recv() => {
                    if let Err(e) = self.send_notifications(event).await {
                        warn!("Failed to handle event: {}", e);
                    }
                }
            };

            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn send_notifications(&self, event: SystemEvent) -> anyhow::Result<()> {
        match event {
            SystemEvent::NetworkConnected { ssid } => {
                Notification::new()
                    .appname("network_module")
                    .summary("Network")
                    .body(format!("Connected to {}", ssid).as_str())
                    .show()?;
            }
            SystemEvent::PowerSupply { is_connected } => {
                if is_connected {
                    Notification::new()
                        .appname("power_supply_module")
                        .summary("Power")
                        .body(format!("Charging").as_str())
                        .show()?;
                } else {
                    Notification::new()
                        .appname("power_supply_module")
                        .summary("Power")
                        .body(format!("Discharging").as_str())
                        .show()?;
                };
            }
            SystemEvent::Device { action, name } => {
                if action == EventType::Add {
                    Notification::new()
                        .appname("device_module")
                        .summary("Device")
                        .body(format!("Device connected: {name}").as_str())
                        .show()?;
                } else if action == EventType::Remove {
                    Notification::new()
                        .appname("device_module")
                        .summary("Device")
                        .body(format!("Device disconnected: {name}").as_str())
                        .show()?;
                };
            }
            x => bail!("Unimplemented event: {x}"),
        };

        Ok(())
    }
}
