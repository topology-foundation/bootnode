# Build image
FROM ubuntu:22.04 as build

# Install all needed for build dependencies
RUN apt-get update -qq && apt-get install -y \
    git \
    cmake \
    g++ \
    pkg-config \
    libssl-dev \
    curl \
    llvm \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy rust toolchain for using fixed rust version
COPY ./rust-toolchain.toml /tmp/rust-toolchain.toml

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install rustup
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --no-modify-path --default-toolchain none

# Install stable toolchain
RUN rustup default stable

# Copy project into build directory
WORKDIR /build
COPY . .

# Compile bootnode
RUN cargo install --locked --path .
RUN cargo build --locked --release

# Runner image
FROM ubuntu:22.04

WORKDIR /app

# Install required dependencies for running binary
RUN apt-get update -qq && apt-get install -y \
    libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy bootnode binary from build image
COPY --from=build /build/target/release/bootnode /app

# Copy bootnode config file
COPY --from=build /build/boot.toml /app

# Set log level
ENV RUST_LOG=info

# Expose bootnode default run port
EXPOSE 2122

# Set entrypoint command for running bootnode
CMD [ "./bootnode" ]
