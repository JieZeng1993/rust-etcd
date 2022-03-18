use std::net::{TcpListener, TcpStream};
use bytes::BytesMut;
use tokio::io::BufWriter;
use crate::connection::Connection;

use crate::state_machine::{Follower, RaftNode};

///从leader变为candidate

fn server() {
    //很多属性都是写死的，后续完善
    let mut all_nodes = vec![
        RaftNode {
            node_id: 1,
            address: "localhost:10011".to_string(),
            connect: None,
            connect_state: false,
        },
        RaftNode {
            node_id: 2,
            address: "localhost:10012".to_string(),
            connect: None,
            connect_state: false,
        },
        RaftNode {
            node_id: 3,
            address: "localhost:10013".to_string(),
            connect: None,
            connect_state: false,
        },
    ];


    let current_raft_node = all_nodes.iter().find(|&raft_node| {
        raft_node.node_id == 1
    }).expect("未找到当前节点的node id");

    let follower = Follower {
        node_id: current_raft_node.node_id,
        current_term: 0,
        voted_for: None,
        logs: vec![],
        commit_index: 0,
        last_applied: 0,
        nodes: all_nodes,
        tcp_listener: TcpListener::bind(current_raft_node.address.clone()).await.unwrap(),
    };

    for raft_node in all_nodes.iter_mut() {
        tokio::spawn(async move {
            if raft_node.connect.is_some() {
                //如果没有连接
                return;
            }

            //没有连接，建立连接
            match TcpStream::connect(raft_node.address.clone()) {
                Ok(tcp_stream) => {
                    raft_node.connect = Option::from(Connection {
                        stream: BufWriter::new(tcp_stream),
                        //包大小
                        buffer: BytesMut::with_capacity(4 * 1024),
                    });

                    loop {

                    }
                }
                Err(_) => {}
            }
        });
    }

    //TODO 发现是否有leader
    //成为候选者
    let candidate = follower.to_candidate();
}