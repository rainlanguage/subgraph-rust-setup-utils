use serial_test::serial;
use subgraph_rust_setup_utils::rpc::RPC;

// #[test]
fn test_rpc_default_creation() {
    let rpc = RPC::default();

    assert_eq!(rpc.get_url(), "http://localhost:8545/");
    assert_eq!(
        rpc.get_provider().url().to_string(),
        "http://localhost:8545/"
    );
}

#[tokio::main]
// #[test]
#[serial]
async fn test_mine() -> anyhow::Result<()> {
    let rpc = RPC::default();

    let before_block = rpc.get_block_number().await?;

    // Mine a block
    rpc.mine_block().await?;

    let after_block = rpc.get_block_number().await?;

    assert_eq!(after_block, before_block + 1, "block not mined");

    Ok(())
}

#[tokio::main]
// #[test]
#[serial]
async fn test_get_block_by_number() -> anyhow::Result<()> {
    let rpc = RPC::default();

    let block_number_0 = rpc.get_block_number().await?;
    let block_data_0 = rpc.get_block_by_number(block_number_0).await?;

    assert_eq!(block_data_0.number.unwrap(), block_number_0);

    // Mine a block
    rpc.mine_block().await?;
    let block_number_1 = rpc.get_block_number().await?;
    assert_eq!(block_number_1, block_number_0 + 1, "block not mined");

    let block_data_1 = rpc.get_block_by_number(block_number_1).await?;

    // Just assert few fields to check that values were obtained correctly
    assert_eq!(block_data_1.number.unwrap(), block_number_1);
    assert_eq!(
        block_data_1.parent_hash,
        block_data_0.hash.unwrap(),
        "not valid parent hash"
    );
    Ok(())
}

#[tokio::main]
// #[test]
#[serial]
async fn test_get_block_by_hash() -> anyhow::Result<()> {
    let rpc = RPC::default();

    let block_number_0 = rpc.get_block_number().await?;
    let block_data_0 = rpc.get_block_by_number(block_number_0).await?;
    let block_data_by_hash_0 = rpc.get_block_by_hash(block_data_0.hash.unwrap()).await?;

    assert_eq!(block_data_0, block_data_by_hash_0);

    // Mine a block
    rpc.mine_block().await?;
    let block_number_1 = rpc.get_block_number().await?;
    assert_eq!(block_number_1, block_number_0 + 1, "block not mined");

    let block_data_1 = rpc.get_block_by_number(block_number_1).await?;
    let block_data_by_hash_1 = rpc.get_block_by_hash(block_data_1.hash.unwrap()).await?;

    assert_eq!(block_data_1, block_data_by_hash_1);
    assert_eq!(
        block_data_by_hash_1.parent_hash,
        block_data_by_hash_0.hash.unwrap(),
        "not valid parent hash"
    );
    Ok(())
}

#[tokio::main]
// #[test]
#[serial]
async fn test_increase_timestamp_native_native_u64() -> anyhow::Result<()> {
    let rpc = RPC::default();

    let block_number_0 = rpc.get_block_number().await?;
    let timestamp_0 = rpc.get_block_by_number(block_number_0).await?.timestamp;

    let time_to_increase = 100u64;
    rpc.increase_block_time(time_to_increase).await?;

    let block_number_1 = rpc.get_block_number().await?;
    assert_eq!(block_number_1, block_number_0 + 1, "block not mined");

    let timestamp_1 = rpc.get_block_by_number(block_number_1).await?.timestamp;

    assert_eq!(timestamp_1, timestamp_0 + time_to_increase);

    Ok(())
}

#[tokio::main]
#[test]
#[serial]
async fn test_get_block_by_tx_hash() -> anyhow::Result<()> {
    todo!("get wallet not implemented to send a valid transaction");
}
