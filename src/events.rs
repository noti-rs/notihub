use derive_more::derive::Display;
use tokio_udev::EventType;

#[derive(Display)]
pub enum SystemEvent {
    #[display("{}, {}", ssid, signal_strenght)]
    NetworkConnected {
        ssid: String,
        signal_strenght: u8,
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
