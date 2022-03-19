FROM rust:1.59 as build

ARG RUN_MODE=production
ENV RUN_MODE=${RUN_MODE}

WORKDIR /caster-api

COPY . .

# Build for release
RUN cargo build --release

# The final release image
FROM debian:bullseye-slim
ARG DUMB_INIT=1.2.5
EXPOSE 3000

ADD https://github.com/Yelp/dumb-init/releases/download/v${DUMB_INIT}/dumb-init_${DUMB_INIT}_x86_64 /usr/local/bin/dumb-init

COPY --from=build /caster-api/target/release/caster-api /usr/src/app/caster-api

RUN groupadd caster \
    && useradd -g caster caster \
    && mkdir -p /usr/src/app

RUN chown -R caster:caster /usr/src/app && \
    chmod +x /usr/local/bin/dumb-init

USER caster
WORKDIR /usr/src/app

ENTRYPOINT ["dumb-init", "--"]

CMD ["sh", "-c", "caster-api"]
