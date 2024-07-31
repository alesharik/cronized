# Cronized
![GitHub Release](https://img.shields.io/github/v/release/alesharik/cronized)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/alesharik/cronized/.github%2Fworkflows%2Fcheck.yaml)

Dead-simple daemon to run Cron jobs in docker.
Monitors job errors, write logs, and exports job info in Prometheus format.

## Example
```dockerfile
# include container to copy binary
FROM alesharik/cronized:bullseye AS cronized

# include your container
FROM debian:bullseye

# copy binary and setup cron
COPY --from=cronized /cronized /cronized
ENV CRONIZED_CRON="* * * * * *"
ENV CRONIZED_CMD="echo 1"

# And run cronized, it will schedule your job
ENTRYPOINT ["/cronized"]
```

## Configuration
1. set `CRONIZED_CRON` env to desired schedule, in cron format
2. write desired command in `CRONIZED_CMD` env

You can also change shell with `CRONIZED_SHELL` env (default is `sh`). Daemon runs commands as `$CRONIZED_SHELL -c "$CRONIZED_CMD"`

Also, you can change working directory with `CRONIZED_WORKDIR` env

## Metrics
Daemon exports metrics on `0.0.0.0:6561` by default.

You can change server address and port with `CRONIZED_METRICS_ADDRESS` environment variable. 
Also, set `CRONIZED_METRICS_ENABLED` to `false` to disable prometheus server

- histogram `cronized_run_time` - Histogram of job run time, in milliseconds
- counter `cronized_errors` - Increases after every job error (exit code != 0)
- gauge `cronized_last_run_is_error` - equals `1` if last run failed, `0` otherwise
- counter `cronized_last_run` - last job run time, in milliseconds timestamp
