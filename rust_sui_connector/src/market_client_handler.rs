use std::{collections::HashMap, sync::Arc};

use crate::DeepBookConnector;

pub(crate) struct SUIConnectors<'a> {
    pub sui_market_clients: HashMap<&'a str, Arc<DeepBookConnector>>
}

// TODO: Replace implicit lifetime
impl SUIConnectors<'_>{
    // No value in get if sui_market_client is public, change this to have getters and setters
    pub fn get_client(&self, retrieval_key: &str) -> Option<&Arc<DeepBookConnector>> {
        self.sui_market_clients.get(retrieval_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_insert_sui_connector(){
        let initial_connector = Arc::new(DeepBookConnector::new("http://localhost".to_string(), 2, 4, "SUI_USDC".to_string()));
        let mut market_connectors = SUIConnectors {
            sui_market_clients: HashMap::new()
        };

        market_connectors.sui_market_clients.insert("test_client", initial_connector);

        assert_eq!(market_connectors.sui_market_clients.len(), 1);
    }
}