use subgraph_rust_setup_utils::rpc::RPC;

#[test]
fn test_rpc_default_creation() {
    let rpc = RPC::default();

    assert_eq!(rpc.get_url(), "http://localhost:8545/");
    assert_eq!(
        rpc.get_provider().url().to_string(),
        "http://localhost:8545/"
    );
}

#[tokio::main]
#[test]
async fn test_mine() -> anyhow::Result<()> {
    // Default RPC
    let rpc = RPC::default();

    let before_block = rpc.get_block_number().await?;

    // Mine a block
    rpc.mine_block().await?;

    let after_block = rpc.get_block_number().await?;

    assert_eq!(after_block, before_block + 1, "block not mined");

    Ok(())
}
