# Use an official Rust runtime as a parent image
FROM rust:latest

ARG PGID
ARG PUID

RUN addgroup --gid $PGID myapp && \
    adduser -u $PUID -G myapp -h /ddns -D myapp

# Set the working directory in the container to /ddns-rust/src/myapp
WORKDIR /ddns-rust/src/myapp

RUN chown -R myapp:myapp /ddns-rust

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

USER myapp:myapp

# Set the startup command to run your binary
CMD ["./target/release/myapp"]