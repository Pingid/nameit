################################################################################
# Create a stage for building the application.
ARG RUST_VERSION=1.73.0
ARG APP_NAME=nameit
FROM rust:${RUST_VERSION}-slim-bullseye AS builder
ARG APP_NAME
WORKDIR /app

RUN \
    --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
    apt-get upgrade && \
    apt-get install -y pkg-config libssl-dev ca-certificates wget

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \ 
    cargo install -f cross

RUN \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \ 
    rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \
    cargo install cargo-leptos

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN sh build.sh

FROM debian:bullseye-slim AS runner

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/$APP_NAME /app/server
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
CMD ["/app/server"]

# # # Build the application.
# # # Leverage a cache mount to /usr/local/cargo/registry/
# # # for downloaded dependencies and a cache mount to /app/target/ for 
# # # compiled dependencies which will speed up subsequent builds.
# # # Leverage a bind mount to the src directory to avoid having to copy the
# # # source code into the container. Once built, copy the executable to an
# # # output directory before the cache mounted /app/target is unmounted.
# # RUN --mount=type=bind,source=src,target=src \
# #     --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
# #     --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
# #     --mount=type=bind,source=style,target=style \
# #     --mount=type=bind,source=public,target=public \
# #     --mount=type=cache,target=/app/target/ \
# #     --mount=type=cache,target=/usr/local/cargo/registry/ \
# #     <<EOF
# # set -e
# # cargo install cargo-leptos
# # rustup target add wasm32-unknown-unknown
# # cargo leptos build --release
# # cp ./target/release/$APP_NAME /app/serverserver
# # cp ./target/site /app/target
# # EOF

# # ################################################################################
# # # Create a new stage for running the application that contains the minimal
# # # runtime dependencies for the application. This often uses a different base
# # # image from the build stage where the necessary files are copied from the build
# # # stage.
# # #
# # # The example below uses the debian bullseye image as the foundation for running the app.
# # # By specifying the "bullseye-slim" tag, it will also use whatever happens to be the
# # # most recent version of that tag when you build your Dockerfile. If
# # # reproducability is important, consider using a digest
# # # (e.g., debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
# # FROM debian:bullseye-slim AS final

# # # Create a non-privileged user that the app will run under.
# # # See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
# # ARG UID=10001
# # RUN adduser \
# #     --disabled-password \
# #     --gecos "" \
# #     --home "/nonexistent" \
# #     --shell "/sbin/nologin" \
# #     --no-create-home \
# #     --uid "${UID}" \
# #     appuser
# # USER appuser

# # # Copy the executable from the "build" stage.
# # COPY --from=build /app /

# # # Expose the port that the application listens on.
# # EXPOSE 3000
# # WORKDIR /app
# # ENV RUST_LOG="info"
# # ENV APP_ENVIRONMENT="production"
# # ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
# # ENV LEPTOS_SITE_ROOT="site"
# # EXPOSE 8080

# # # What the container should run when it is started.
# # CMD ["./server"]
