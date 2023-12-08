use ethers::signers::Signer;
use subgraph_rust_setup_utils::wallets::WalletHandler;

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
    let client_1 = wallets.get_client(1).await?;
    let client_18 = wallets.get_client(18).await?;
    let client_19 = wallets.get_client(19).await?;

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
    let client_1 = wallets.get_client(wallet_1.clone()).await?;
    let client_18 = wallets.get_client(wallet_18.clone()).await?;
    let client_19 = wallets.get_client(wallet_19.clone()).await?;

    assert_eq!(wallet_0.address(), client_0.address());
    assert_eq!(wallet_1.address(), client_1.address());
    assert_eq!(wallet_18.address(), client_18.address());
    assert_eq!(wallet_19.address(), client_19.address());

    Ok(())
}
