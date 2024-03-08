#!/usr/bin/env sh

mc-log-monitor --log-level "$LOG_LEVEL" --delay "$STARTUP_DELAY" --monitor-config /config/monitor.toml
