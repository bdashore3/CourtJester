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
pub struct JapanResult {
    pub id: i64,
    pub title: String,
    pub main_picture: MainPicture,
    pub synopsis: String,
    pub mean: Option<f64>,
    pub status: String,
    pub num_episodes: Option<i64>,
    pub num_volumes: Option<i64>,
    pub num_chapters: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainPicture {
    pub medium: String,
    pub large: String,
}

/*
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
*/

#[derive(Debug, Deserialize)]
pub struct GifResult {
    pub url: String,
    pub media: Vec<HashMap<String, TenorMedia>>,
}

#[derive(Debug, Deserialize)]
pub struct TenorMedia {
    pub url: String,
}
