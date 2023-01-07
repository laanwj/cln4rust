FROM ubuntu:20.04
LABEL mantainer="Vincenzo Palazzo vincenzopalazzodev@gmail.com"

ENV TZ=Europe/Minsk
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Ubuntu utils
RUN apt-get update && apt-get install -y \
    software-properties-common  \
    build-essential \
    curl wget

RUN apt-get update

## Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install bitcoin core and lightningd (last version)
RUN add-apt-repository ppa:luke-jr/bitcoincore
RUN apt-get update && apt-get install -y bitcoind jq libsodium-dev libpq-dev

ENV CLIGHTNING_VERSION=22.11.1

RUN wget https://github.com/ElementsProject/lightning/releases/download/v$CLIGHTNING_VERSION/clightning-v$CLIGHTNING_VERSION-Ubuntu-20.04.tar.xz && \
    tar -xvf clightning-v$CLIGHTNING_VERSION-Ubuntu-20.04.tar.xz -C /opt && cd /opt && \
    mv usr clightning-v$CLIGHTNING_VERSION

RUN rm -r clightning-v$CLIGHTNING_VERSION-Ubuntu-20.04.tar.xz

ENV PATH=/opt/clightning-v$CLIGHTNING_VERSION/bin:$PATH

WORKDIR workdir
COPY sandbox .
COPY . /workdir/code

CMD ["./entrypoint.sh"]
