pub struct Config {
    pub network: NetworkConfig,
    pub power_supply: PSConfig,
    pub device: DeviceConfig,
}

pub struct NetworkConfig {
    pub enabled: bool,
}

pub struct PSConfig {
    pub enabled: bool,
}

pub struct DeviceConfig {
    pub enabled: bool,
}
