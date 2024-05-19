# Use an official Rust runtime as a parent image
FROM rust:latest

ARG PGID
ARG PUID

RUN groupadd -g $PGID mygroup && \
    useradd -u $PUID -g mygroup -d /ddns -m myuser

# Set the working directory in the container to /ddns-rust/src/myapp

COPY /ddns-rust /app
WORKDIR /app
RUN chown -R myuser:mygroup /app

RUN cargo build --release

# Set the startup command to run your binary
USER myuser:mygroup
CMD ["./target/release/ddns-rust"]