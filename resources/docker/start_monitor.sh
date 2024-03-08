#!/usr/bin/env sh

mc-log-monitor --log-level "$LOG_LEVEL" --delay-start "$STARTUP_DELAY" --monitor-config /config/monitor.toml
