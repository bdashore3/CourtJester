pub mod cmd_data;
pub mod commands;
pub mod errors;

use serde::{Serialize, Deserialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeResult {
    pub mal_id: u64,
    pub url: String,
    pub image_url: String,
    pub title: String,
    pub airing: bool,
    pub synopsis: String,
    pub episodes: i64,
    pub score: f64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MangaResult {
    pub mal_id: u64,
    pub url: String,
    pub image_url: String,
    pub title: String,
    pub publishing: bool,
    pub synopsis: String,
    pub chapters: i64,
    pub volumes: i64,
    pub score: f64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
