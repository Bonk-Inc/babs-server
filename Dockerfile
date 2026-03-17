#####################################################################
## Build Backend
####################################################################
FROM rust:1.94-slim-trixie AS build

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

# set the working directory.
WORKDIR /babs-server

# copy backend project files to the working directory.
COPY ./ .

# build the backend to a executable.
RUN cargo build --target x86_64-unknown-linux-gnu --release -p babs-server

#####################################################################
## Final image
####################################################################
FROM debian:trixie-slim

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libpq5 ca-certificates
RUN update-ca-certificates

WORKDIR /babs-server

# Copy our build
COPY --from=build /babs-server/target/x86_64-unknown-linux-gnu/release/babs-server ./

# Set appuser info
ENV USER=babs-server
ENV UID=666

# Create new non-root user
RUN useradd \
    --system \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
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
ENTRYPOINT ["/babs-server/babs-server"]