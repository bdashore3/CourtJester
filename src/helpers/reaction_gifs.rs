use reqwest::Url;
use serde::Deserialize;
use serenity::{client::Context, framework::standard::CommandResult, model::id::GuildId};

use crate::{structures::GifResult, PubCreds, ReactionImageCache, ReqwestClient};

#[derive(Debug, Deserialize)]
struct Response {
    results: Vec<GifResult>,
}

pub async fn fetch_gifs(
    ctx: &Context,
    search: &str,
    amount: usize,
    filter: &str,
) -> CommandResult<Vec<GifResult>> {
    let (reqwest_client, tenor_key) = {
        let data = ctx.data.read().await;
        let reqwest_client = data.get::<ReqwestClient>().cloned().unwrap();
        let tenor_key = data
            .get::<PubCreds>()
            .unwrap()
            .get("tenor")
            .cloned()
            .unwrap();

        (reqwest_client, tenor_key)
    };

    let url = Url::parse_with_params(
        "https://api.tenor.com/v1/search",
        &[
            ("q", search),
            ("key", tenor_key.as_str()),
            ("limit", &format!("{}", amount)),
            ("contentfilter", filter),
        ],
    )?;

    let resp = reqwest_client
        .get(url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(resp.results)
}

pub async fn add_to_cache(ctx: &Context, guild_id: GuildId, key: String, url: String) {
    let image_cache = ctx
        .data
        .read()
        .await
        .get::<ReactionImageCache>()
        .cloned()
        .unwrap();

    let cached_url = image_cache.get(&(guild_id, key.to_owned()));

    match cached_url {
        Some(cached_url) => {
            if cached_url.value() != &url {
                drop(cached_url);

                image_cache.insert((guild_id, key), url);
            }
        }
        None => {
            image_cache.insert((guild_id, key), url);
        }
    }
}

pub async fn check_image_cache(
    ctx: &Context,
    guild_id: GuildId,
    search_key: String,
    mut gifs: Vec<GifResult>,
) -> Vec<GifResult> {
    let image_cache = ctx
        .data
        .read()
        .await
        .get::<ReactionImageCache>()
        .cloned()
        .unwrap();

    let cached_url = match image_cache.get(&(guild_id, search_key)) {
        Some(image_struct) => image_struct,
        None => return gifs,
    };

    if let Some(index) = gifs.iter().position(|gif| &gif.url == cached_url.value()) {
        gifs.remove(index);
    };

    gifs
}
