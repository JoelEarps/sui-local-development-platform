use connector_model::connector::{market_builder::MarketBuilder, market_type::{MarketBuilderParameters, MarketType}};

use async_trait::async_trait;
use connector_model::orderbook::OrderBook;


use crate::market_data_collector::deepbook_orderbook::DeepBookV3OrderBookData;

/// # Testing
/// ## Integration tests
/// async fn fetch_orderbook is tested here rust_sui_connector/tests
/// It held no value to unit test this implementation currently as it would require providing a mock implementation
/// Once the scanner is implemented this may change however I imagine there will be a mock connector 
struct DeepBookConnector {
    orderbook_url: String,
    depth: usize,
    length: usize,
    market_pair: String
}

impl DeepBookConnector {
    pub(crate) fn new(base_url: String, depth: usize, length: usize, market_pair: String) -> Self {
        let full_api_url = format!("{}{}?level={}&depth={}",base_url, market_pair, 2, depth);
        Self {
            orderbook_url: full_api_url,
            depth,
            length,
            market_pair
        }
    }
}

#[async_trait]
impl MarketBuilder<u64> for DeepBookConnector {
   async fn fetch_orderbook(&self, params: &MarketBuilderParameters<u64>) -> anyhow::Result<OrderBook> {
        let resp = reqwest::get(&self.orderbook_url)
        .await?
        .json::<DeepBookV3OrderBookData>()
        .await?;
        Ok(OrderBook::try_from(resp)?)
   }

   fn market_type(&self) -> MarketType { 
        todo!()
    }

}

// TODO: Move this test to be an integration tests using test containers and mock endpoint, containing deterministic data
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use futures::stream::BoxStream;
    use futures::StreamExt; 
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn fetch_orderbook_deepbook(){
        let test_deepbook_connector = DeepBookConnector::new("https://deepbook-indexer.mainnet.mystenlabs.com/orderbook/".to_string(), 2 , 4, "SUI_USDC".to_string());
        let test = Arc::new(test_deepbook_connector);
        let mut output_stream: BoxStream<'static, anyhow::Result<OrderBook>> = test.build_orderbook_stream(MarketBuilderParameters { orders_limit: 15, convertion_params: None}, 1000, true).await;

        let mut orderbook_results: Vec<OrderBook> = Vec::new();

        // Is there a better way to do this, take doesn't seem to work?
        for _ in 0..5 {
            if let Some(orderbook) = output_stream.next().await {
                orderbook_results.push(orderbook.unwrap());
            } else {
                panic!("Stream did not produce any output!");
            }
        }

        assert_eq!(orderbook_results.len(), 5);

        // TODO: What are the criteria that deem this a success
        // 5 results from 5 iterations
        // Mock data and therefore get results from each set of data?
    }
}