use ethers::core::k256::ecdsa::SigningKey;
use ethers::providers::{Http, Provider};
use ethers::signers::{Signer, Wallet};
use subgraph_rust_setup_utils::WalletHandler;
#[test]
fn test_get_wallet() -> anyhow::Result<()> {
    // Wallets using junk mnemonic
    let wallets = WalletHandler::default();

    let wallet_0 = wallets.get_wallet(0)?;
    let wallet_1 = wallets.get_wallet(1)?;
    let wallet_18 = wallets.get_wallet(18)?;
    let wallet_19 = wallets.get_wallet(19)?;

    assert_eq!(
        format!("{:?}", wallet_0.address()).to_lowercase(),
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_lowercase()
    );
    assert_eq!(
        format!("{:?}", wallet_1.address()).to_lowercase(),
        "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_lowercase()
    );
    assert_eq!(
        format!("{:?}", wallet_18.address()).to_lowercase(),
        "0xdD2FD4581271e230360230F9337D5c0430Bf44C0".to_lowercase()
    );
    assert_eq!(
        format!("{:?}", wallet_19.address()).to_lowercase(),
        "0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199".to_lowercase()
    );

    Ok(())
}

#[tokio::main]
#[test]
async fn test_get_client_with_index() -> anyhow::Result<()> {
    let wallets = WalletHandler::default();

    let wallet_0 = wallets.get_wallet(0)?;
    let wallet_1 = wallets.get_wallet(1)?;
    let wallet_18 = wallets.get_wallet(18)?;
    let wallet_19 = wallets.get_wallet(19)?;

    let client_0 = wallets.get_client(0).await?;
    let client_1 = wallets.get_client(&1).await?;
    let client_18 = wallets.get_client(18).await?;
    let client_19 = wallets.get_client(&19).await?;

    assert_eq!(wallet_0.address(), client_0.address());
    assert_eq!(wallet_1.address(), client_1.address());
    assert_eq!(wallet_18.address(), client_18.address());
    assert_eq!(wallet_19.address(), client_19.address());

    Ok(())
}

#[tokio::main]
#[test]
async fn test_get_client_with_wallet() -> anyhow::Result<()> {
    let wallets = WalletHandler::default();

    let wallet_0 = wallets.get_wallet(0)?;
    let wallet_1 = wallets.get_wallet(1)?;
    let wallet_18 = wallets.get_wallet(18)?;
    let wallet_19 = wallets.get_wallet(19)?;

    let client_0 = wallets.get_client(wallet_0.clone()).await?;
    let client_1 = wallets.get_client(&wallet_1).await?;
    let client_18 = wallets.get_client(wallet_18.clone()).await?;
    let client_19 = wallets.get_client(&wallet_19).await?;

    assert_eq!(wallet_0.address(), client_0.address());
    assert_eq!(wallet_1.address(), client_1.address());
    assert_eq!(wallet_18.address(), client_18.address());
    assert_eq!(wallet_19.address(), client_19.address());

    Ok(())
}

#[tokio::main]
#[test]
async fn test_get_client_with_option_wallet() -> anyhow::Result<()> {
    let wallets = WalletHandler::default();

    let wallet_none: Option<Wallet<SigningKey>> = None;
    let wallet_0 = wallets.get_wallet(0)?;
    let wallet_9 = wallets.get_wallet(9)?;

    let client_with_none_wallet = wallets.get_client(wallet_none).await?;
    let client_with_some_wallet = wallets.get_client(&wallet_9).await?;

    assert_eq!(wallet_0.address(), client_with_none_wallet.address());
    assert_eq!(wallet_9.address(), client_with_some_wallet.address());

    Ok(())
}

#[tokio::main]
#[test]
async fn test_create_wallet_handler() -> anyhow::Result<()> {
    // Arbitrary mnemonics
    let mnemonic_0 = "wool craft draw left side fit oak rotate brush stereo foster leader";
    let wallets_handler_0 = WalletHandler::new(mnemonic_0, None);

    let wallet_0 = wallets_handler_0.get_wallet(0)?;
    let client_0 = wallets_handler_0.get_client(wallet_0.clone()).await?;
    assert_eq!(wallet_0.address(), client_0.address());
    assert_eq!(
        wallets_handler_0.get_provider().url().to_string(),
        "http://localhost:8545/"
    );

    let mnemonic_1 = "ticket palm can arrest mutual mom cash blush fiber panda coast vendor";
    let wallets_handler_1 = WalletHandler::new(mnemonic_1, None);

    let wallet_1 = wallets_handler_1.get_wallet(0)?;
    let client_1 = wallets_handler_1.get_client(wallet_1.clone()).await?;
    assert_eq!(wallet_1.address(), client_1.address());
    assert_eq!(
        wallets_handler_1.get_provider().url().to_string(),
        "http://localhost:8545/"
    );

    Ok(())
}

#[tokio::main]
#[test]
async fn test_create_wallet_handler_with_provider() -> anyhow::Result<()> {
    // Arbitrary mnemonics
    let mnemonic_0 = "coffee worry panda wreck online salt mean limb vibrant issue erode busy";
    let url_provide_0 = "https://rpc-mumbai.maticvigil.com/";
    let provider_0 = Provider::<Http>::try_from(url_provide_0)?;

    let wallets_handler_0 = WalletHandler::new(mnemonic_0, Some(provider_0));

    let wallet_0 = wallets_handler_0.get_wallet(0)?;
    let client_0 = wallets_handler_0.get_client(wallet_0.clone()).await?;
    assert_eq!(wallet_0.address(), client_0.address());
    assert_eq!(
        wallets_handler_0.get_provider().url().to_string(),
        url_provide_0
    );

    Ok(())
}
