use crate::state_machine::{Follower, RaftNode};

///从leader变为candidate

fn server() {
    //很多属性都是写死的，后续完善
    let all_nodes = vec![
        RaftNode {
            node_id: 1,
            address: "localhost:10011".to_string(),
            connect: None,
            connect_state: false
        },
        RaftNode {
            node_id: 2,
            address: "localhost:10012".to_string(),
            connect: None,
            connect_state: false
        },
        RaftNode {
            node_id: 3,
            address: "localhost:10013".to_string(),
            connect: None,
            connect_state: false
        },
    ];

    let follower = Follower {
        node_id: 1,
        current_term: 0,
        voted_for: None,
        logs: vec![],
        commit_index: 0,
        last_applied: 0,
        nodes: all_nodes,
    };
    //TODO 发现是否有leader
    //成为候选者
    let candidate = follower.to_candidate();
}