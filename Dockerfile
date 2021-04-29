# Dockerfile for `act` (local github actions runner)
FROM rust:1.51

RUN apt-get update && apt-get install -y curl

# For Cypress
RUN \
    apt-get update && \
    apt-get install -y libgtk2.0-0 libgtk-3-0 libgbm-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2 libxtst6 xauth xvfb
RUN \
    apt-get update && apt-get install -y npm

# For headless browser tests
RUN apt-get install -y firefox-esr chromium

# Install NPM
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get update && apt-get install -y nodejs

# Everything below is just to speed things up.
# It'll be installed by the github actions workflow as required.

# Install wasm-opt
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_101/binaryen-version_101-x86_64-linux.tar.gz | tar -xz
RUN cp binaryen-version_101/bin/wasm-opt /usr/bin/

RUN cargo install cargo-udeps
RUN cargo install mdbook
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN cargo install wasm-pack
RUN cargo install xargo

RUN rustup component add clippy rustfmt
RUN rustup target add wasm32-unknown-unknown
