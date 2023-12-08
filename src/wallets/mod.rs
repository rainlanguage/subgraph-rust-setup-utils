use ethers::{
    core::k256::ecdsa::SigningKey,
    prelude::SignerMiddleware,
    providers::Middleware,
    signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet, WalletError},
};

use crate::rpc::RPC;
use ethers::providers::{Http, Provider};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum WalletInput {
    Index(u32),
    Wallet(Wallet<SigningKey>),
}

impl From<&WalletInput> for WalletInput {
    fn from(input_ref: &WalletInput) -> Self {
        input_ref.to_owned()
    }
}
impl From<Option<WalletInput>> for WalletInput {
    fn from(input_option: Option<WalletInput>) -> Self {
        match input_option {
            Some(input) => input,
            None => WalletInput::default(),
        }
    }
}

impl From<&u32> for WalletInput {
    fn from(num: &u32) -> Self {
        WalletInput::Index(num.to_owned())
    }
}

impl From<u32> for WalletInput {
    fn from(num: u32) -> Self {
        WalletInput::Index(num)
    }
}

impl From<&Wallet<SigningKey>> for WalletInput {
    fn from(wallet: &Wallet<SigningKey>) -> Self {
        WalletInput::Wallet(wallet.to_owned())
    }
}

impl From<Option<Wallet<SigningKey>>> for WalletInput {
    fn from(wallet_option: Option<Wallet<SigningKey>>) -> Self {
        match wallet_option {
            Some(wallet) => WalletInput::Wallet(wallet),
            None => WalletInput::default(),
        }
    }
}
impl From<Wallet<SigningKey>> for WalletInput {
    fn from(wallet: Wallet<SigningKey>) -> Self {
        WalletInput::Wallet(wallet)
    }
}

impl Default for WalletInput {
    /// The default value it will be assumed to be a numeric 0 as index
    fn default() -> Self {
        WalletInput::Index(0)
    }
}

#[derive(Clone)]
pub struct WalletHandler {
    wallet_builder: MnemonicBuilder<English>,
    provider: Provider<Http>,
}

impl Default for WalletHandler {
    /// Get a WalletHandler with default options. This means that will use the test mnemonic
    /// `test test test test test test test test test test test junk` and provider evm node
    /// at `http://localhost:8545/`
    fn default() -> Self {
        let mnemonic = "test test test test test test test test test test test junk";

        WalletHandler {
            wallet_builder: MnemonicBuilder::<English>::default().phrase(mnemonic),
            provider: RPC::default().get_provider().to_owned(),
        }
    }
}

impl WalletHandler {
    /// Create a WalletHandler from a mnemonic and an optional provider
    pub fn new(mnemonic: &str, provider: Option<Provider<Http>>) -> Self {
        let provider_field = match provider {
            Some(provider) => provider,
            None => RPC::default().get_provider().to_owned(),
        };

        WalletHandler {
            wallet_builder: MnemonicBuilder::<English>::default().phrase(mnemonic),
            provider: provider_field,
        }
    }

    /// Get a wallet from the given index
    pub fn get_wallet(&self, index: u32) -> anyhow::Result<Wallet<SigningKey>, WalletError> {
        self.wallet_builder.clone().index(index)?.build()
    }

    /// Get a client from either a numeric value or an existing Wallet
    pub async fn get_client<T>(
        &self,
        wallet_or_index: T,
    ) -> anyhow::Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>
    where
        T: Into<WalletInput> + Clone,
    {
        let wallet_or_index: WalletInput = wallet_or_index.into();

        let wallet = match wallet_or_index {
            WalletInput::Index(index) => self.get_wallet(index)?,
            WalletInput::Wallet(wallet) => wallet,
        };

        let client = Arc::new(SignerMiddleware::new(
            self.get_provider().clone(),
            wallet.with_chain_id(self.get_provider().get_chainid().await?.as_u64()),
        ));

        Ok(client)
    }

    // Getter method for the Provider in the WalletHandler
    pub fn get_provider(&self) -> &Provider<Http> {
        &self.provider
    }
}
