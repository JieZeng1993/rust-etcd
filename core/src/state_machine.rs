use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;
use crate::connection::Connection;

pub struct RaftNode {
    pub node_id: i64,
    pub address: String,
    //连接信息
    pub connect: Option<Connection>,
    pub connect_state: bool,
}


///跟随者
pub struct Follower {
    ///节点id
    pub node_id: i64,
    ///通用持久化状态，服务器已知最新的任期（再服务器首次启动的时候初始化为0，单调递增）
    pub current_term: i64,
    ///通用持久化状态，当前任期内收到选票的候选者id 如果没有投给任何候选者 则为空
    pub voted_for: Option<i64>,
    ///通用持久化状态，日志条目；每个条目包含了用于状态机的命令，以及领导者接受到该条目时的任期（第一个索引为1）
    pub logs: Vec<String>,
    ///通用易失性状态，已知已提交的最高的日志条目的索引（初始值为0，单调递增）
    pub commit_index: i64,
    ///通用易失性状态，已经倍应用到状态机的最高的日志条目的索引（初始值为0，单调递增）
    pub last_applied: i64,
    ///所有节点
    pub nodes: Arc<Mutex<Vec<RaftNode>>>,
    pub tcp_listener: TcpListener,
}

///候选者
pub struct Candidate {
    ///节点id
    pub node_id: i64,
    ///通用持久化状态，服务器已知最新的任期（再服务器首次启动的时候初始化为0，单调递增）
    pub current_term: i64,
    ///通用持久化状态，当前任期内收到选票的候选者id 如果没有投给任何候选者 则为空
    pub voted_for: Option<i64>,
    ///通用持久化状态，日志条目；每个条目包含了用于状态机的命令，以及领导者接受到该条目时的任期（第一个索引为1）
    pub logs: Vec<String>,
    ///通用易失性状态，已知已提交的最高的日志条目的索引（初始值为0，单调递增）
    pub commit_index: i64,
    ///通用易失性状态，已经倍应用到状态机的最高的日志条目的索引（初始值为0，单调递增）
    pub last_applied: i64,
    ///所有节点
    pub nodes: Arc<Vec<RaftNode>>,
    ///投票统计
    pub votes: Vec<i64>,
    pub tcp_listener: TcpListener,
}

///领导者
pub struct Leader {
    ///节点id
    pub node_id: i64,
    ///通用持久化状态，服务器已知最新的任期（再服务器首次启动的时候初始化为0，单调递增）
    pub current_term: i64,
    ///通用持久化状态，当前任期内收到选票的候选者id 如果没有投给任何候选者 则为空
    pub voted_for: Option<i64>,
    ///通用持久化状态，日志条目；每个条目包含了用于状态机的命令，以及领导者接受到该条目时的任期（第一个索引为1）
    pub logs: Vec<String>,
    ///通用易失性状态，已知已提交的最高的日志条目的索引（初始值为0，单调递增）
    pub commit_index: i64,
    ///通用易失性状态，已经倍应用到状态机的最高的日志条目的索引（初始值为0，单调递增）
    pub last_applied: i64,
    ///领导者易失性状态，对于每一台服务器，发送到该服务器的下一条日志条目的索引（初始值为领导者最后的日志条目索引+1）
    pub next_index: Vec<i64>,
    ///领导者易失性状态，对于每一台服务器，已知的已经复制到该服务器的最高日志条目的索引（初始值为0，单调递增）
    pub match_index: Vec<i64>,
    ///所有节点
    pub nodes: Vec<RaftNode>,
    pub tcp_listener: TcpListener,
}

impl Follower {
    ///没有leader时，申请成为候选者
    pub fn to_candidate(mut self) -> Candidate {
        let mut candidate = Candidate {
            node_id: self.node_id,
            //变为候选者，自增当前的任期号
            current_term: self.current_term + 1,
            voted_for: self.voted_for,
            logs: self.logs,
            commit_index: self.commit_index,
            last_applied: self.last_applied,
            nodes: self.nodes,
            votes: vec![],
            tcp_listener: self.tcp_listener
        };

        //变为候选者自己给自己投票
        candidate.votes.push(candidate.node_id);

        //TODO 向各个节点发送数据
        for raft_node in &candidate.nodes {}
        //重置选举超时计时器
        loop {
            let (tx, rx) = channel();
            //100毫秒
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(()) => {}
                Err(RecvTimeoutError::Timeout) => {
                    //超时，再次进行成为候选者，term+1，发送所有消息，并修改选举超时时间
                }
                Err(RecvTimeoutError::Disconnected) => {
                    //通道关闭，执行节点关闭逻辑
                }
            }
        }

        candidate
    }
}