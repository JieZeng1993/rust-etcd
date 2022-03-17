use crate::state_machine::Follower;

///从leader变为candidate

fn server() {
    let follower = Follower {
        node_id: 1,
        current_term: 0,
        voted_for: None,
        logs: vec![],
        commit_index: 0,
        last_applied: 0,
        //后续通过
        node_count: 3
    };
    //TODO 发现是否有leader
    //成为候选者
    let candidate = follower.to_candidate();
}