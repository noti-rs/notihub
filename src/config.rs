pub struct Config {
    pub network: CommonConfig,
    pub power_supply: CommonConfig,
    pub device: CommonConfig,
}

pub struct CommonConfig {
    pub enabled: bool,
}
