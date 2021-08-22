use reqwest::Error as ReqwestError;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::fmt::Write;
use std::time::Duration;

use crate::{
    helpers::embed_store,
    structures::{cmd_data::ReqwestClient, errors::JesterError, AnimeResult, MangaResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    data: Vec<ResultType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultType {
    Anime(AnimeResult),
    Manga(MangaResult),
}

impl ResultType {
    fn unwrap_anime(&self) -> Option<&AnimeResult> {
        if let ResultType::Anime(a) = self {
            Some(a)
        } else {
            None
        }
    }

    fn unwrap_manga(&self) -> Option<&MangaResult> {
        if let ResultType::Manga(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

#[command]
async fn anime(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .say(
                ctx,
                JesterError::MissingError("anime title for me to work with!"),
            )
            .await?;

        return Ok(());
    }

    let results = match fetch_info(ctx, "anime", args.rest()).await {
        Ok(info) => info.data,
        Err(e) => {
            println!("Jikan anime fetch error!: {}", e);

            msg.channel_id
                .say(
                    ctx,
                    "Couldn't find your request on MAL! The error is down below.",
                )
                .await?;

            return Err(e);
        }
    };

    let animes = results
        .iter()
        .filter_map(ResultType::unwrap_anime)
        .collect::<Vec<_>>();

    let result_string = animes
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (num, anime)| {
            let _ = writeln!(&mut acc, "{}. {}\n", num + 1, anime.title);
            acc
        });

    let result_embed = embed_store::get_result_embed(&result_string);

    let sent_message = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.0 = result_embed.0;
                e
            })
        })
        .await?;

    while let Ok(value) = ask_for_results(ctx, msg).await {
        let index = value as usize;

        if let Some(anime) = animes.get(index - 1) {
            let anime_embed = embed_store::get_anime_embed(anime);

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.0 = anime_embed.0;
                        e
                    })
                })
                .await?;

            break;
        }
    }

    sent_message.delete(ctx).await?;

    Ok(())
}

#[command]
async fn manga(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .say(
                ctx,
                JesterError::MissingError("manga title for me to work with!"),
            )
            .await?;

        return Ok(());
    }

    let results = match fetch_info(ctx, "manga", args.rest()).await {
        Ok(info) => info.data,
        Err(e) => {
            println!("Jikan manga fetch error!: {}", e);

            msg.channel_id
                .say(
                    ctx,
                    "Couldn't find your request on MAL! The error is down below.",
                )
                .await?;

            return Err(e);
        }
    };

    let mangas = results
        .iter()
        .filter_map(ResultType::unwrap_manga)
        .collect::<Vec<_>>();

    let result_string = mangas
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (num, manga)| {
            let _ = writeln!(&mut acc, "{}. {}\n", num + 1, manga.title);
            acc
        });

    let result_embed = embed_store::get_result_embed(&result_string);

    let sent_message = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.0 = result_embed.0;
                e
            })
        })
        .await?;

    while let Ok(value) = ask_for_results(ctx, msg).await {
        let index = value as usize;

        if let Some(manga) = mangas.get(index - 1) {
            let manga_embed = embed_store::get_manga_embed(manga);

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.0 = manga_embed.0;
                        e
                    })
                })
                .await?;

            break;
        }
    }

    sent_message.delete(ctx).await?;

    Ok(())
}

async fn ask_for_results(ctx: &Context, msg: &Message) -> CommandResult<isize> {
    let channel_id = msg.channel_id;

    let result = msg
        .author
        .await_reply(ctx)
        .filter(move |given_msg| given_msg.channel_id == channel_id)
        .timeout(Duration::from_secs(30))
        .await;

    match result {
        Some(recieved_msg) => {
            if recieved_msg.content == "abort" {
                let _ = recieved_msg.channel_id.say(ctx, "Aborting...").await;

                return Err("Aborted".into());
            }

            match recieved_msg.content.parse::<isize>() {
                Ok(num) => Ok(num),
                Err(_) => Ok(-1),
            }
        }
        None => {
            let _ = channel_id.say(ctx, "Timed out").await;

            Err("Timeout".into())
        }
    }
}

async fn fetch_info(ctx: &Context, search_type: &str, search: &str) -> CommandResult<Response> {
    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestClient>()
        .cloned()
        .unwrap();

    let url = Url::parse_with_params(
        &format!("https://api.jikan.moe/v4/{}", search_type),
        &[("q", search), ("limit", "5")],
    )?;

    let resp = match reqwest_client.get(url).send().await {
        Ok(res) => res,
        Err(e) => {
            let casted_error = e as ReqwestError;

            if casted_error.is_timeout() {
                return Err("Request timed out".into());
            } else {
                return Err(casted_error.into());
            }
        }
    };

    let json = resp.json::<Response>().await?;

    Ok(json)
}

pub async fn japan_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "anime <title>: Searches for an anime's information from the title \n\n",
        "manga <title>: Searches for a manga's information from the title"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Japan Help");
                e.description("Description: Commands that deal with japanese media");
                e.field("Commands", content, false);
                e
            })
        })
        .await;
}
