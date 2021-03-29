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

# Allows user to specify additional packages to include at build time
ARG ADDITIONAL_PACKAGE

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates ${ADDITIONAL_PACKAGE} \
    && addgroup -S app && adduser -S -g app app

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