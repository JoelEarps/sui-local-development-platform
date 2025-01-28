FROM ubuntu:latest AS setup
ENV SUI_VERSION=1.41.0
ARG INDEXER_DB_URL

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    tar \
    postgresql \
    iputils-ping \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js (LTS version) and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm

WORKDIR /app

RUN curl -L -o sui-package.tgz https://github.com/MystenLabs/sui/releases/download/testnet-v${SUI_VERSION}/sui-testnet-v1.41.0-ubuntu-aarch64.tgz

RUN tar -xzf sui-package.tgz

RUN rm sui-package.tgz

ENV PATH="$PATH:/app" 

COPY ./smart_contract_simple /app/smart_contract_simple

FROM setup AS fresh_start_with_indexer

# RUN RUST_LOG=off sui_node=info sui start --with-faucet --force-regenesis --with-indexer --with-graphql --pg-host postgres-sui-indexer --pg-user admin --pg-password password --pg-db-name sui_indexer
# RUN sui client new-env --alias local --rpc http://127.0.0.1:9000
# RUN sui client switch --env local
# RUN sui client faucet

# CMD ["env", "RUST_LOG=off,sui_node=info", "sui", "start", "--with-faucet", "--force-regenesis", "--with-indexer", "--with-graphql", "--pg-host" , "postgres-sui-indexer", "--pg-user", "admin", "--pg-password", "password", "--pg-db-name", "sui_indexer"] 

COPY sui_start_script.sh /app/sui_start_script.sh

RUN chmod +x /app/sui_start_script.sh

# Run the script
CMD ["/app/sui_start_script.sh"]

FROM setup AS fresh_start_no_indexer

# No GraphQL for local example data
# CMD ["env", "RUST_LOG=off,sui_node=info", "sui", "start", "--with-faucet", "--force-regenesis"] 
CMD ["env", "RUST_LOG=off,sui_node=info", "sui", "start", "--with-faucet", "--force-regenesis"]

# sui client new-env --alias local --rpc http://127.0.0.1:9000
# RUN sui client switch --env local
# RUN sui client faucet


