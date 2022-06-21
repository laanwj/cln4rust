FROM ubuntu:20.04
LABEL mantainer="Vincenzo Palazzo vincenzopalazzodev@gmail.com"

ENV TZ=Europe/Minsk
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Ubuntu utils
RUN apt-get update && apt-get install -y \
    software-properties-common  \
    build-essential \
    curl

RUN apt-get update

## Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install bitcoin core and lightningd (last version)
RUN add-apt-repository ppa:luke-jr/bitcoincore
RUN apt-get update  && apt-get install -y bitcoind jq
RUN add-apt-repository -u ppa:lightningnetwork/ppa
RUN apt-get update  && apt-get install -y lightningd

WORKDIR workdir
COPY sandbox .
COPY . /workdir/code

CMD ["./entrypoint.sh"]