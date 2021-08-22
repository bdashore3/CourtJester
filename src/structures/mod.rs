pub mod cmd_data;
pub mod commands;
pub mod errors;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CommitResponse {
    pub sha: String,
    pub html_url: String,
}

#[derive(Default, Debug)]
pub struct SysInfo {
    pub shard_latency: String,
    pub memory: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeResult {
    pub mal_id: u64,
    pub url: String,
    pub images: HashMap<String, JikanImage>,
    pub title: String,
    pub title_english: String,
    pub airing: bool,
    pub synopsis: String,
    pub episodes: i64,
    pub score: f64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JikanImage {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MangaResult {
    pub mal_id: u64,
    pub url: String,
    pub images: HashMap<String, JikanImage>,
    pub title: String,
    pub title_english: String,
    pub publishing: bool,
    pub synopsis: String,
    pub chapters: Option<u64>,
    pub volumes: Option<u64>,
    pub scored: f64,
}

#[derive(Debug, Deserialize)]
pub struct GifResult {
    pub url: String,
    pub media: Vec<HashMap<String, TenorMedia>>,
}

#[derive(Debug, Deserialize)]
pub struct TenorMedia {
    pub url: String,
}
