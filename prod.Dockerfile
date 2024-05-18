# Use an official Rust runtime as a parent image
FROM rust:latest

ARG PGID
ARG PUID

RUN groupadd -g $PGID mygroup && \
    useradd -u $PUID -g mygroup -d /ddns -m myuser

# Set the working directory in the container to /ddns-rust/src/myapp
WORKDIR /app

RUN chown -R myuser:mygroup /app
# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

USER myuser:mygroup

# Set the startup command to run your binary
CMD cargo run