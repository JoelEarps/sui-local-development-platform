# SUI Docker Local Test Env

_Note_: This is currently a playground - just trying to get it working for now!

Purpose is to run SUI and make an application for it, with the purpose of creating a test environment

Am I Making an L2 Application?

`docker build -t local-sui-platform .`

`docker run -it local-sui-platform /bin/bash`

Running this should start a container with the application running - you can then go in and have a play

if you run 

```

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

