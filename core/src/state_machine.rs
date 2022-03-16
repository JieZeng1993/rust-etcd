///跟随者
pub struct Follower {
    ///服务器已知最新的任期（再服务器首次启动的时候初始化为0，单调递增）
    pub current_term: i64,
    ///当前任期内收到选票的候选者id 如果没有投给任何候选者 则为空
    pub voted_for: Option<i32>,
    ///日志条目；每个条目包含了用于状态机的命令，以及领导者接受到该条目时的任期（第一个索引为1）
    pub logs: Vec<String>,
}

///候选者
pub struct Candidate {}

///领导者
pub struct Leader {}