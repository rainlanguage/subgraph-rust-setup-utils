use subgraph_rust_setup_utils::rpc::RPC;

#[test]
fn test_rpc_default_creation() {
    //b
    let rpc = RPC::default();

    assert_eq!(rpc.get_url(), "http://localhost:8545/");
    assert_eq!(
        rpc.get_provider().url().to_string(),
        "http://localhost:8545/"
    );
}

#[test]
fn test_a() {}
