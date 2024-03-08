#!/usr/bin/env sh

sleep "$STARTUP_DELAY"
mc-log-monitor --log-level "$LOG_LEVEL" --monitor-config /config/monitor.toml
