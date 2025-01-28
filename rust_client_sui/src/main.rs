use sui_sdk::{types::base_types::SuiAddress, SuiClientBuilder};
use std::str::FromStr;

mod events;
use events::event_handler::say_hello;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let sui_client = SuiClientBuilder::default()
        .build("http://localhost:9000") // local network address
        .await?;
    println!("Sui testnet version: {}", sui_client.api_version());


    // Here we are looking at the address on the test net where owned objects should be
    // The looking at the owned objects which are..
    // The address is obtained by doing the following:
    let address = SuiAddress::from_str("0x2f8fa895085bf96e40b9f49256070cd05f7247cd422b804451c71923288ca8af")?;
    let owned_objects = sui_client
        .read_api()
        .get_owned_objects(address, None, None, None)
        .await?;

    println!("{:?}", owned_objects);

    let total_balance = sui_client
    .coin_read_api()
    .get_all_balances(address)
    .await?;
    println!("The balances for all coins owned by address: {address} are {:?}", total_balance);

    say_hello();
    Ok(())
}

/* trustless swap example, our rust app has to do the following:
1. Set up Event Listener for Backend Indexer
2. Collect events that occur\
3. Set up an API to query none event objects from the indexer
Multi threaded application
4. Create a client to send commands

These stages have been taken directly from the following URL:
https://docs.sui.io/guides/developer/app-examples/trustless-swap#smart-contracts
*/
