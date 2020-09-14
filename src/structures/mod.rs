pub mod cmd_data;
pub mod commands;
pub mod errors;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitResponse {
    pub sha: String,
    pub html_url: String
}

#[derive(Default, Debug)]
pub struct SysInfo {
    pub shard_latency: String,
    pub memory: f32
}
