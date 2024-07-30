use crate::config::Config;
use ::metrics::{counter, gauge, histogram};
use anyhow::bail;
use chrono::{Local, Utc};
use cron::Schedule;
use std::process::Stdio;
use std::str::FromStr;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;
use tracing::{debug, error, info, span, Instrument, Level};
use tracing_subscriber::EnvFilter;

mod config;
mod metrics;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::parse()?;
    info!(config = ?config, "Using config");

    metrics::install(&config.metrics)?;

    for time in Schedule::from_str(&config.cron)?.upcoming(Local) {
        run_cycle(&config)
            .instrument(span!(Level::INFO, "command", command = ?config.command))
            .await?;

        let delta = time - Local::now();
        debug!("Sleeping {:?}", &delta);
        sleep(Duration::from_millis(delta.num_milliseconds() as u64)).await;
    }

    bail!("Should not end")
}

async fn run_cycle(config: &Config) -> anyhow::Result<()> {
    debug!("Running command");

    counter!("cronized_last_run").absolute(Utc::now().timestamp_millis() as u64);
    let start = Utc::now();

    let code = (if let Some(ref workdir) = config.workdir {
        Command::new(&config.shell)
            .arg("-c")
            .arg(&config.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(workdir)
            .spawn()?
    } else {
        Command::new(&config.shell)
            .arg("-c")
            .arg(&config.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    })
    .wait()
    .await?;

    let delta = Duration::from_millis((Utc::now() - start).num_milliseconds() as u64);
    debug!(delta = ?delta, status = ?code, "Command completed");
    histogram!("cronized_run_time").record(delta.as_millis() as f64);

    if code.success() {
        gauge!("cronized_last_run_is_error").set(0);
    } else {
        counter!("cronized_errors").increment(1);
        gauge!("cronized_last_run_is_error").set(1);
        error!(status = ?code, "Command failed");
    }

    Ok(())
}
