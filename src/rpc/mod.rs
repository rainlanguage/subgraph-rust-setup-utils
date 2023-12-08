use ethers::providers::{Http, Provider};
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

    fn create_provider(url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(url).expect("could not instantiate HTTP Provider")
    }
}
