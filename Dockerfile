# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install brotli
RUN \
    apt-get update -y && \
    apt-get install -y brotli

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Install node using fnm
RUN curl -fsSL https://fnm.vercel.app/install | bash
ENV PATH="/root/.local/share/fnm:$PATH"
RUN eval "$(fnm env --use-on-cd)" && \
    fnm install 20 && \
    fnm use 20 && \
    npm install

# Build the app
RUN cargo leptos build --release -vv

# Compress assets
RUN ls -d /app/target/site/pkg/* | xargs brotli

FROM rustlang/rust:nightly-bullseye as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/nameit /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/nameit"]
