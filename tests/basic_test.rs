use subgraph_rust_setup_utils::add;

#[test]
fn test_add_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add(-2, -3), -5);
}

#[test]
fn test_add_diff_numbers() {
    assert_eq!(add(-2, 3), 1);
}
