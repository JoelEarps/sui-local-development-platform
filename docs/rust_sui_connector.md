# Rust SUI connector

Located at `../rust_sui_connector`

## Requirements

1. Poll market data from the deepbookv3 Http API
2. This market data must be polled at 1 seconds
3. The data to be polled must be validated
4. Errors relating to scenarios
5. Configuration arguments such as API url must be passed in via config to enable testing.\

## Testing Scenarios 

BDD and Gherkin Sytnax using test containers with WireMock to provide mock data for error and none error scenarios

## Data Schemas and Schema Validation

How is data validated

Data to be collected and


## Intgration Tests

Set up Wire Mock for docker image integration tests
Can we use the benchmark feature of rust to look at the processing and E2E times of calling the
