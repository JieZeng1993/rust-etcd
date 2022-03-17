struct TestNode {
    node_id: i64,
    address: String,
}

#[test]
fn test() {
    let test_nodes = vec![
        TestNode {
            node_id: 0,
            address: "0".to_string(),
        },
        TestNode {
            node_id: 1,
            address: "1".to_string(),
        },
        TestNode {
            node_id: 1,
            address: "1".to_string(),
        },
    ];

    for x in &test_nodes {

    }
}