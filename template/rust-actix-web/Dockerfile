FROM openfaas/of-watchdog:0.7.2 as watchdog

FROM rust:1.51 as builder

WORKDIR /home/rust

# Copy all the sources
COPY ./ ./

# Build and install application
RUN cargo install --path ./main

FROM debian:buster-slim as runner

# Allows user to specify additional packages to include at build time
ARG ADDITIONAL_PACKAGE

# Install packages and add non-root user
RUN apt update && apt-get install -y curl ca-certificates ${ADDITIONAL_PACKAGE} --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && addgroup --system app && adduser --system --ingroup app app

ENV USER=app

# Copy compiled function from builder 
COPY --from=builder /usr/local/cargo/bin/main /usr/local/bin/main
RUN chmod +x /usr/local/bin/main

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Set up watchdog for HTTP mode
ENV fprocess="main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

EXPOSE 8080

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]