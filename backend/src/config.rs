use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub _callsign: String,
    pub _locator: String,
    pub _sdr_enabled: bool,
    pub _sdr_device: String,
    pub _sdr_sample_rate: u32,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file
        dotenvy::dotenv().ok();

        // Use ConfigBuilder for modern API
        let settings = config::Config::builder()
            // Set defaults
            .set_default("port", 3000)?
            .set_default("host", "127.0.0.1")?
            .set_default("_callsign", "N0CALL")?
            .set_default("_locator", "FN31")?
            .set_default("sdr_enabled", false)?
            .set_default("sdr_device", "rtlsdr")?
            .set_default("sdr_sample_rate", 2400000)?
            // Merge environment variables
            .add_source(config::Environment::with_prefix("HAMSHACK"))
            .build()?;

        settings.try_deserialize()
    }
}
