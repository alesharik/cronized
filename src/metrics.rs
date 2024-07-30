use crate::config::MetricsConfig;
use anyhow::Result;
use metrics::{describe_counter, describe_gauge, describe_histogram, Unit};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use std::net::SocketAddr;
use std::time::Duration;
use tracing::{debug, info};

pub fn install(config: &MetricsConfig) -> Result<()> {
    let mut builder = PrometheusBuilder::new().idle_timeout(
        MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
        Some(Duration::from_secs(10)),
    );
    if config.enabled {
        info!(
            "Will create prometheus metrics endpoint at {}",
            &config.address
        );
        builder = builder.with_http_listener(config.address.parse::<SocketAddr>()?);
    }
    builder.install()?;

    describe_histogram!(
        "cronized_run_time",
        Unit::Milliseconds,
        "Histogram of job run time"
    );
    describe_counter!("cronized_errors", Unit::Count, "Job error count");
    describe_gauge!("cronized_last_run_is_error", "1 if last run errored");
    describe_counter!(
        "cronized_last_run",
        Unit::Milliseconds,
        "When job last run time is"
    );

    debug!("Metrics set up");
    Ok(())
}
