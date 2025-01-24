FROM ubuntu:latest
ENV SUI_VERSION=1.41.0

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    tar \
    iputils-ping \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN curl -L -o sui-package.tgz https://github.com/MystenLabs/sui/releases/download/testnet-v${SUI_VERSION}/sui-testnet-v1.41.0-ubuntu-aarch64.tgz

RUN tar -xzf sui-package.tgz

RUN rm sui-package.tgz

ENV PATH="$PATH:/app" 

CMD ["env", "RUST_LOG=off,sui_node=info", "sui", "start", "--with-faucet", "--force-regenesis"]