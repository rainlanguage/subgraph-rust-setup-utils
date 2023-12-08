use ethers::providers::{Http, Provider};
use ethers::types::{Block, H256, U64};
use reqwest::Client;
use std::str::FromStr;

/// RPC for interactions with a rpc url with their methods in an easy way
pub struct RPC {
    url: String,
    provider: Provider<Http>,
}

impl Default for RPC {
    /// Create a default RPC using as rpc url a local node at `http://localhost:8545/` to send the requests
    fn default() -> Self {
        let default_url = "http://localhost:8545/";

        RPC {
            url: String::from(default_url),
            provider: Self::create_provider(default_url),
        }
    }
}

impl RPC {
    /// Create a RPC using the `url` to send the requests
    pub fn new(url: &str) -> Self {
        RPC {
            url: String::from(url),
            provider: Self::create_provider(url),
        }
    }

    /// Create a RPC using an existing `provider`
    pub fn from_provider(provider: Provider<Http>) -> Self {
        RPC {
            url: provider.url().to_string(),
            provider,
        }
    }

    // Getter method for the URL in the RPC
    pub fn get_url(&self) -> &str {
        &self.url
    }

    // Getter method for the Provider in the RPC
    pub fn get_provider(&self) -> &Provider<Http> {
        &self.provider
    }

    /// Try to mine a block using `evm_mine`
    pub async fn mine_block(&self) -> anyhow::Result<()> {
        let json_rpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "evm_mine",
            "params": [],
        });

        self.send_request(json_rpc_request).await?;

        Ok(())
    }

    /// Get the current block on the network
    pub async fn get_block_number(&self) -> anyhow::Result<U64> {
        let json_rpc_request = serde_json::json!({
          "jsonrpc": "2.0",
          "id": 1,
          "method": "eth_blockNumber",
          "params": [],
        });

        let block_number = match self.send_request(json_rpc_request).await {
            Ok(data) => {
                let value_data = data.as_str().unwrap().to_string();

                U64::from_str(&value_data)?
            }
            Err(err) => return Err(err),
        };

        Ok(block_number)
    }

    pub async fn get_block_by_number(&self, block_number: U64) -> anyhow::Result<Block<H256>> {
        let json_rpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": [block_number, false],
        });

        let response = self.send_request(json_rpc_request).await?;
        let block: Block<H256> = serde_json::from_value(response)?;

        Ok(block)
    }

    async fn send_request(
        &self,
        json_data_request: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let provider = self.get_provider();

        let response: reqwest::Response = Client::new()
            .post(provider.url().as_str())
            .json(&json_data_request)
            .send()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;

            match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&text) {
                Ok(parsed_json) => {
                    match parsed_json.get("error") {
                        Some(err_value) => match err_value.get("message") {
                            // Return the error message obtained by the RPC node
                            Some(err_message) => Err(anyhow::anyhow!("{}", err_message)),
                            None => Err(anyhow::anyhow!(
                                "Error - No 'message' field in the 'error' object"
                            )),
                        },
                        None => {
                            // If no error key, we can assume that the response was succesfull
                            // Safe to unwrap since if no error, the result field will exist
                            return Ok(parsed_json.get("result").unwrap().clone());
                        }
                    }
                }
                // Return the JSON parse error
                Err(err) => Err(anyhow::anyhow!("Error parsing the response JSON: {}", err)),
            }
        } else {
            Err(anyhow::anyhow!(
                "Failed to communicate with the RPC node. HTTP status code: {}",
                response.status()
            ))
        }
    }

    fn create_provider(url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(url).expect("could not instantiate HTTP Provider")
    }
}
