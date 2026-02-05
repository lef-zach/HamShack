use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub callsign: String,
    pub locator: String,
    pub sdr_enabled: bool,
    pub sdr_device: String,
    pub sdr_sample_rate: u32,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file
        dotenvy::dotenv().ok();
        
        // Use ConfigBuilder for modern API
        let settings = config::Config::builder()
            // Set defaults
            .set_default("port", 3000)?
            .set_default("host", "0.0.0.0")?
            .set_default("callsign", "N0CALL")?
            .set_default("locator", "FN31")?
            .set_default("sdr_enabled", false)?
            .set_default("sdr_device", "rtlsdr")?
            .set_default("sdr_sample_rate", 2400000)?
            // Merge environment variables
            .add_source(config::Environment::with_prefix("HAMSHACK"))
            .build()?;
        
        settings.try_deserialize()
    }
}