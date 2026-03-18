# Versions
ARG RUST_IMAGE_VERSION=1.94-slim-trixie
ARG DEBIAN_IMAGE_VERSION=trixie-slim

# Set appuser info
ARG USER=babs-website
ARG USER_ID=32767

#####################################################################
## Build Backend
####################################################################
FROM rust:${RUST_IMAGE_VERSION} AS build

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

# set the working directory.
WORKDIR /var/www/html

# copy backend project files to the working directory.
COPY ./ .

# build the backend to a executable.
RUN cargo build --target x86_64-unknown-linux-gnu --release -p babs-server

#####################################################################
## Final image
####################################################################
FROM debian:${DEBIAN_IMAGE_VERSION}

ARG USER
ARG USER_ID

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libpq5 ca-certificates
RUN update-ca-certificates

WORKDIR /var/www/html

# Copy our build
COPY --from=build /var/www/html/target/x86_64-unknown-linux-gnu/release/babs-server ./

# Create new non-root user
RUN useradd \
    --system \
    --shell "/sbin/nologin" \
    --uid "${USER_ID}" \
    "${USER}"

# Create data folder
RUN mkdir data

# Set file permissions
RUN chmod +rw *
RUN chown -R ${USER}:${USER} *

# Use an unprivileged user.
USER ${USER}

EXPOSE 8080

# Start the server
ENTRYPOINT ["./babs-server"]