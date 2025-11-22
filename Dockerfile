#####################################################################
## Build Backend
####################################################################
FROM rust:1.88-slim-bookworm AS backend-build

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

# set the working directory.
WORKDIR /bonk-inc-backend

# copy backend project files to the working directory.
COPY ./ .

# build the backend to a executable.
RUN cargo build --target x86_64-unknown-linux-gnu --release -p babs-server

#####################################################################
## Final image
####################################################################
FROM debian:bookworm-slim

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libpq5 ca-certificates
RUN update-ca-certificates

# Import from builder.
COPY --from=backend-build /etc/passwd /etc/passwd
COPY --from=backend-build /etc/group /etc/group

WORKDIR /bonk-inc-backend

# Copy our build
COPY --from=backend-build /bonk-inc-backend/target/x86_64-unknown-linux-gnu/release/babs-server ./

# create appuser
ENV USER=bonk-inc-backend
ENV UID=32767

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Create data folder
RUN mkdir data

# Set file permissions
RUN chmod +rw *
RUN chown -R bonk-inc-backend:bonk-inc-backend *

# Use an unprivileged user.
USER bonk-inc-backend:bonk-inc-backend

EXPOSE 8080

CMD ["/bonk-inc-backend/babs-server"]