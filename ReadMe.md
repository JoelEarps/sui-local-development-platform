# SUI Docker Local Test Env

_Note_: This is currently a playground - just trying to get it working for now!

Purpose is to run SUI and make an application for it, with the purpose of creating a test environment

Am I Making an L2 Application?

`docker build -t local-sui-platform .`

`docker run -it local-sui-platform /bin/bash`

Running this should start a container with the application running - you can then go in and have a play

if you run in the container. When port forwarding you should be able to do this from outside the container as well.

```bash

curl --location --request POST 'http://127.0.0.1:9000' \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sui_getTotalTransactionBlocks",
  "params": []
}'

```

You should be able to receive `{"jsonrpc":"2.0","id":1,"result":"10018"}` to verify things are running.

## First time set up

Config file ["/root/.sui/sui_config/client.yaml"] doesn't exist, do you want to connect to a Sui Full node server [y/N]

Connects you to the testnet full node

What is a full node?

`sui client switch --env local`

## Setting up with pre-exsiting config

## TODOs

Client Health Check commands, Readiness - make sure local env
`sui client active-env`

Set up scripts for new setup - seeding data and post indexer connection steps
The installation of test data is done using turbo repo, TS and test containers, we do not want to do that
Get coins and check for coins
Set up a database migration script and data initialisation script for where you do not need fake data
Run a PostGresDB in a different container and get things started ✅
Indexer and RPC graphQL containers rather than contained all in one infrastructure?

```bash

-pg-port <PG_PORT>
          Port for the Indexer Postgres DB. Default port is 5432
          
          [default: 5432]

      --pg-host <PG_HOST>
          Hostname for the Indexer Postgres DB. Default host is localhost
          
          [default: localhost]

      --pg-db-name <PG_DB_NAME>
          DB name for the Indexer Postgres DB. Default DB name is sui_indexer
          
          [default: sui_indexer]

      --pg-user <PG_USER>
          DB username for the Indexer Postgres DB. Default username is postgres
          
          [default: postgres]

      --pg-password <PG_PASSWORD>
          DB password for the Indexer Postgres DB. Default password is postgrespw
          
          [default: postgrespw]

```

We have got to the point now where we need to seed the data and start making a client

They suggest TS but we are looking to work in rust

