FROM lukemathwalker/cargo-chef:latest as chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /mc-log-monitor

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY ./ .
RUN cargo build --release --bin mc-log-monitor

# Final image
FROM debian:12-slim

RUN mkdir /mc-log-monitor
WORKDIR /mc-log-monitor

ENV PATH=/mc-log-monitor:$PATH \
LOG_LEVEL=Info

ADD resources/docker/start_monitor.sh /mc-log-monitor/
COPY --from=builder /mc-log-monitor/target/release/mc-log-monitor /mc-log-monitor
VOLUME /config
VOLUME /logs

CMD ["/bin/sh","start_monitor.sh"]
