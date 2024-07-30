#[derive(Debug)]
pub struct Config {
    pub cron: String,
    pub command: String,
    pub shell: String,
    pub workdir: Option<String>,
    pub metrics: MetricsConfig,
}

impl Config {
    pub fn parse() -> anyhow::Result<Config> {
        Ok(Config {
            cron: std::env::var("CRONIZED_CRON").expect("Env 'CRONIZED_CRON' required"),
            command: std::env::var("CRONIZED_CMD").expect("Env 'CRONIZED_CMD' required"),
            shell: std::env::var("CRONIZED_SHELL")
                .ok()
                .unwrap_or_else(|| "sh".to_string()),
            workdir: std::env::var("CRONIZED_WORKDIR").ok(),
            metrics: MetricsConfig::parse()?,
        })
    }
}

#[derive(Debug)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub address: String,
}

impl MetricsConfig {
    pub fn parse() -> anyhow::Result<MetricsConfig> {
        Ok(MetricsConfig {
            enabled: std::env::var("CRONIZED_METRICS_ENABLED")
                .ok()
                .map(|s| {
                    s.parse::<bool>()
                        .expect("Failed to parse 'CRONIZED_METRICS_ENABLED'! Should be bool")
                })
                .unwrap_or(true),
            address: std::env::var("CRONIZED_METRICS_ADDRESS")
                .ok()
                .unwrap_or_else(|| "0.0.0.0:6561".to_string()),
        })
    }
}
