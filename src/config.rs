pub struct Config {
    pub network: NetworkConfig,
    pub power: PowerConfig,
    pub device: DeviceConfig,
    pub custom: Vec<CustomConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            network: NetworkConfig {
                wifi: Some(WifiConfig {
                    connected: Some(EventConfig {
                        enabled: true,
                        icon: None,
                        summary: Some(TextConfig {
                            format: Some(String::from("Network")),
                        }),
                        body: Some(TextConfig {
                            format: Some(String::from("Connected to $ssid")),
                        }),
                        replace: Some(true),
                    }),
                    disconnected: None,
                    show_signal_strength: Some(false),
                    show_ssid: Some(true),
                }),
                ethernet: None,
            },
            power: PowerConfig {
                charging: Some(EventConfig {
                    enabled: false,
                    icon: None,
                    summary: Some(TextConfig {
                        format: Some(String::from("$charging")),
                    }),
                    body: None,
                    replace: Some(true),
                }),
                battery_level: None,
            },
            device: DeviceConfig {
                connected: Some(EventConfig {
                    enabled: true,
                    icon: None,
                    summary: Some(TextConfig {
                        format: Some(String::from("Device")),
                    }),
                    body: Some(TextConfig {
                        format: Some(String::from("Connected: $name")),
                    }),
                    replace: Some(true),
                }),
                disconnected: None,
            },
            custom: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct CustomConfig {
    pub name: Option<String>,
    pub module: Option<EventConfig>,
}

#[derive(Clone)]
pub struct PowerConfig {
    pub charging: Option<EventConfig>,
    pub battery_level: Option<EventConfig>,
}

#[derive(Clone)]
pub struct DeviceConfig {
    pub connected: Option<EventConfig>,
    pub disconnected: Option<EventConfig>,
}

#[derive(Clone)]
pub struct NetworkConfig {
    pub wifi: Option<WifiConfig>,
    pub ethernet: Option<EthernetConfig>,
}

#[derive(Clone)]
pub struct WifiConfig {
    pub connected: Option<EventConfig>,
    pub disconnected: Option<EventConfig>,
    pub show_signal_strength: Option<bool>,
    pub show_ssid: Option<bool>,
}

#[derive(Clone)]
pub struct EthernetConfig {
    pub connected: Option<EventConfig>,
    pub disconnected: Option<EventConfig>,
}

#[derive(Clone)]
pub struct EventConfig {
    pub enabled: bool,
    pub icon: Option<String>,
    pub summary: Option<TextConfig>,
    pub body: Option<TextConfig>,
    pub replace: Option<bool>,
}

#[derive(Clone)]
pub struct TextConfig {
    pub format: Option<String>,
}
