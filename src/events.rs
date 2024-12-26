use derive_more::derive::Display;
use tokio_udev::EventType;

#[derive(Display)]
pub enum SystemEvent {
    NetworkConnected {
        ssid: String,
    },
    PowerSupply {
        is_connected: bool,
    },

    #[display("{}, {}", action, name)]
    Device {
        action: EventType,
        name: String,
    },
}
