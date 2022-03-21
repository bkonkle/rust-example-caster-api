FROM rust:1.59 as build

WORKDIR /caster-api

COPY . .

# Build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=private,target=/caster-api/target \
    cargo build --release && \
    # Copy executable out of the cache so it is available in the final image.
    cp target/release/caster-api ./caster-api

# The final release image
FROM debian:bullseye-slim

ARG RUN_MODE=production
ENV TZ=Etc/UTC \
    RUN_MODE=${RUN_MODE} \
    RUST_LOG=info

EXPOSE 3000

COPY --from=build /caster-api/caster-api /usr/src/app/caster-api

# Install some Debian packages
RUN apt update && \
    apt-get -y install --no-install-recommends ca-certificates tzdata && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # Set up the app user
    groupadd caster && \
    useradd -g caster caster && \
    mkdir -p /usr/src/app && \
    chown -R caster:caster /usr/src/app

USER caster
WORKDIR /usr/src/app

COPY config config/

CMD ["./caster-api"]
