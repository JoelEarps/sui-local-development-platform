#!/bin/bash

# Set environment variables
export RUST_LOG=off
export SUI_NODE=info

# Start the Sui service with all the required options
echo "Starting Sui node with faucet and indexer..."
sui start --with-faucet --force-regenesis --with-indexer --with-graphql \
  --pg-host postgres-sui-indexer \
  --pg-user admin \
  --pg-password password \
  --pg-db-name sui_indexer

# Add a new environment alias for the Sui client - doesn't work due to input required
echo "Creating new Sui client environment..."
sui client new-env --alias local --rpc http://127.0.0.1:9000

# Switch to the new environment
echo "Switching to the new Sui client environment..."
sui client switch --env local

# Request tokens from the faucet
echo "Requesting tokens from the faucet..."
sui client faucet