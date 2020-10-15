use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
        Args
    }
};
use std::time::Duration;
use std::fmt::Write;
use reqwest::Url;
use serde::{Serialize, Deserialize};

use crate::{
    helpers::embed_store,
    structures::{
        AnimeResult, MangaResult,
        cmd_data::ReqwestClient,
        errors::JesterError
    }
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    results: Vec<ResultType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultType {
    Anime(AnimeResult),
    Manga(MangaResult)
}

impl ResultType {
    fn unwrap_anime(&self) -> Option<&AnimeResult> {
        if let ResultType::Anime(a) = self { Some(a) } else { None }
    }

    fn unwrap_manga(&self) -> Option<&MangaResult> {
        if let ResultType::Manga(m) = self { Some(m) } else { None }
    }
}

#[command]
async fn anime(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("anime title for me to work with!")).await?;

        return Ok(())
    }

    let results = match fetch_info(ctx, "anime", args.rest()).await {
        Ok(info) => info.results,
        Err(_) => {
            msg.channel_id.say(ctx, "Couldn't find your request on MAL!").await?;

            return Ok(())
        }
    };

    let animes = results.iter().filter_map(ResultType::unwrap_anime).collect::<Vec<_>>();

    let result_string = animes.iter().enumerate().fold(String::new(), |mut acc, (num, anime)| {
        let _ = writeln!(&mut acc, "{}. {}\n", num + 1, anime.title);
        acc
      });

    let result_embed = embed_store::get_result_embed(&result_string);

    let sent_message = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.0 = result_embed.0;
            e
        })
    }).await?;

    loop {
        let value = match ask_for_results(ctx, msg).await {
            Ok(wrapped_value) => wrapped_value,
            Err(_) => break
        };

        let index = value as usize;

        if index > 0 && index <= animes.len() {
            let anime_embed = embed_store::get_anime_embed(animes[index - 1]);

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.0 = anime_embed.0;
                    e
                })
            }).await?;

            break
        }
    }

    sent_message.delete(ctx).await?;

    Ok(())
}

#[command]
async fn manga(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, JesterError::MissingError("manga title for me to work with!")).await?;

        return Ok(())
    }

    let results = match fetch_info(ctx, "manga", args.rest()).await {
        Ok(info) => info.results,
        Err(_) => {
            msg.channel_id.say(ctx, "Couldn't find your request on MAL!").await?;

            return Ok(())
        }
    };

    let mangas = results.iter().filter_map(ResultType::unwrap_manga).collect::<Vec<_>>();

    let result_string = mangas.iter().enumerate().fold(String::new(), |mut acc, (num, manga)| {
        let _ = writeln!(&mut acc, "{}. {}\n", num + 1, manga.title);
        acc
      });

    let result_embed = embed_store::get_result_embed(&result_string);

    let sent_message = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.0 = result_embed.0;
            e
        })
    }).await?;
    
    loop {
        let value = match ask_for_results(ctx, msg).await {
            Ok(wrapped_value) => wrapped_value,
            Err(_) => break
        };

        let index = value as usize;

        if index > 0 && index <= mangas.len() {
            let manga_embed = embed_store::get_manga_embed(&mangas[index - 1]);

            msg.channel_id.send_message(ctx, |m| {
                m.embed(|e| {
                    e.0 = manga_embed.0;
                    e
                })
            }).await?;

            break
        }
    }

    sent_message.delete(ctx).await?;

    Ok(())
}

async fn ask_for_results(ctx: &Context, msg: &Message) -> CommandResult<isize> {
    let channel_id = msg.channel_id;

    let result = msg.author.await_reply(ctx)
        .filter(move |given_msg| given_msg.channel_id == channel_id)
        .timeout(Duration::from_secs(30)).await;

    match result {
        Some(recieved_msg) => {
            if recieved_msg.content == "abort" {
                let _ = recieved_msg.channel_id.say(ctx, "Aborting...").await;

                return Err("Aborted".into())
            }

            match recieved_msg.content.parse::<isize>() {
                Ok(num) => return Ok(num),
                Err(_) => return Ok(-1)
            }
        },
        None => {
            let _ = channel_id.say(ctx, "Timed out").await;

            return Err("Timeout".into())
        }
    } 
}

async fn fetch_info(ctx: &Context, search_type: &str, search: &str) -> CommandResult<Response> {
    let reqwest_client = ctx.data.read().await
        .get::<ReqwestClient>().cloned().unwrap();

    let url = Url::parse_with_params(&format!("https://api.jikan.moe/v3/search/{}", search_type),
            &[
                ("q", search),
                ("limit", "5")
            ])?;

    let resp = reqwest_client.get(url)
        .send().await?
        .json::<Response>().await?;

    Ok(resp)
}

pub async fn japan_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "anime <title>: Searches for an anime's information from the title \n\n",
        "manga <title>: Searches for a manga's information from the title");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Japan Help");
            e.description("Description: Commands that deal with japanese media");
            e.field("Commands", content, false);
            e
        })
    }).await;
}
