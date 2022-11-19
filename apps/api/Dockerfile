FROM debian:bullseye-slim

ARG RUN_MODE=production
ENV TZ=Etc/UTC \
    RUN_MODE=${RUN_MODE} \
    RUST_LOG=info

EXPOSE 3000

# Install some Debian packages
RUN apt update && \
    apt-get -y install --no-install-recommends ca-certificates tzdata && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # Set up the app user
    groupadd caster && \
    useradd -g caster caster && \
    mkdir -p /usr/src/app && \
    chown caster:caster /usr/src/app

COPY --chown=caster:caster ../../target/release/caster-api /usr/src/app/caster-api
COPY --chown=caster:caster ../../config/*.toml /usr/src/app/config/

USER caster
WORKDIR /usr/src/app

CMD ["./caster-api"]
