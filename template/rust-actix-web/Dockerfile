FROM openfaas/of-watchdog:0.7.2 as watchdog

FROM rust:1.51-alpine as builder

# Add alpine-sdk so dynamic linking for musl will work
RUN apk --no-cache add alpine-sdk

WORKDIR /home/rust

# Copy all the sources
COPY ./ ./

# Build and install application
RUN cargo install --path ./main

FROM alpine:3.12 as runner

# Add non-root user
RUN addgroup -S app && adduser -S -g app app

# Copy function binary
COPY --from=builder /usr/local/cargo/bin/main /usr/local/bin/main
RUN chown -R app:app /usr/local/bin/main \ 
    && chmod +x /usr/local/bin/main

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chown -R app:app /usr/bin/fwatchdog \ 
    && chmod +x /usr/bin/fwatchdog

# Switch to non-root user
USER app

# Build args for setting env variables
ARG RUST_LOG=INFO
ARG WORKER_POOL_SIZE=1
ARG JSON_MAX_SIZE=4096

# Set up watchdog for HTTP mode
ENV fprocess="main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

EXPOSE 8080

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]