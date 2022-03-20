FROM debian:bullseye-slim

ARG DUMB_INIT=1.2.5
ARG RUN_MODE=production
ENV RUN_MODE=${RUN_MODE}

EXPOSE 3000

ADD https://github.com/Yelp/dumb-init/releases/download/v${DUMB_INIT}/dumb-init_${DUMB_INIT}_x86_64 /usr/local/bin/dumb-init

# RUN apt update && \
#     apt-get -y install --no-install-recommends libssl-dev && \
#     apt-get clean && \
#     rm -rf /var/lib/apt/lists/*

COPY target/release /usr/src/app

RUN groupadd caster && \
    useradd -g caster caster && \
    mkdir -p /usr/src/app && \
    chown -R caster:caster /usr/src/app && \
    chmod +x /usr/local/bin/dumb-init

USER caster
WORKDIR /usr/src/app

ENTRYPOINT ["dumb-init", "--"]

CMD ["sh", "-c", "caster-api"]
