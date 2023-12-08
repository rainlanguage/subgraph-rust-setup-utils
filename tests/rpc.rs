use subgraph_rust_setup_utils::rpc::RPC;

#[test]
fn test_rpc() {
    //b
    let rpc = RPC::default();

    assert_eq!(rpc.get_url(), "http://localhost:8545");
}

#[test]
fn test_a() {
}
