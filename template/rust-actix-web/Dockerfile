FROM openfaas/of-watchdog:0.7.2 as watchdog

FROM rust:1.37-alpine as builder

WORKDIR /home/rust

# Copy all the sources
COPY ./ ./

RUN cargo build --workspace --target x86_64-unknown-linux-musl --release

FROM alpine:3.11 as runner

# Allows user to specify additional packages to include at build time
ARG ADDITIONAL_PACKAGE

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates ${ADDITIONAL_PACKAGE} \
    && addgroup -S app && adduser -S -g app app

ENV USER=app

# Copy compiled function from builder 
COPY --from=builder /home/rust/target/release/main /usr/bin/main
RUN chmod +x /usr/bin/main

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Set up watchdog for HTTP mode
ENV fprocess="/usr/bin/main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

EXPOSE 8080

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]