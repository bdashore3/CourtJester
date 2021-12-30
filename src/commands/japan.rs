use reqwest::{Error as ReqwestError, Url};
use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::fmt::Write;
use std::time::Duration;

use crate::{
    helpers::{command_utils::get_command_name, embed_store},
    structures::{errors::JesterError, JapanResult},
    PubCreds, ReqwestClient,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    data: Vec<Datum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    node: JapanResult,
}

#[command]
#[aliases("manga")]
async fn anime(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let command_name = get_command_name(ctx, msg).await;

    if args.is_empty() {
        msg.channel_id
            .say(
                ctx,
                JesterError::MissingError(&format!("{} title for me to work with!", command_name)),
            )
            .await?;

        return Ok(());
    }

    let results = match fetch_info(ctx, command_name, args.rest()).await {
        Ok(info) => info.data,
        Err(e) => {
            println!("MAL fetch error!: {}", e);

            msg.channel_id
                .say(
                    ctx,
                    "Couldn't find your request on MAL! \nThe error is down below.",
                )
                .await?;

            return Err(e);
        }
    };

    if results.is_empty() {
        msg.channel_id
            .say(
                ctx,
                "There are no results! Try some different search terms!",
            )
            .await?;

        return Ok(());
    }

    let result_string = results
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (num, anime)| {
            let _ = writeln!(&mut acc, "{}. {}\n", num + 1, anime.node.title);
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

        if let Some(result) = results.get(index - 1) {
            let japan_embed = embed_store::get_anime_embed(&result.node);

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.0 = japan_embed.0;
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
    let (reqwest_client, mal_key) = {
        let data = ctx.data.read().await;
        let reqwest_client = data.get::<ReqwestClient>().cloned().unwrap();
        let tenor_key = data.get::<PubCreds>().unwrap().get("mal").cloned().unwrap();

        (reqwest_client, tenor_key)
    };

    let optional_fields = if search_type == "anime" {
        "num_episodes"
    } else {
        "num_chapters,num_volumes"
    };

    let url = Url::parse_with_params(
        &format!("https://api.myanimelist.net/v2/{}", search_type),
        &[
            ("q", search),
            ("limit", "5"),
            (
                "fields",
                &format!(
                    "id,title,main_picture,synopsis,mean,status,{}",
                    optional_fields
                ),
            ),
        ],
    )?;

    let resp = match reqwest_client
        .get(url)
        .header("X-MAL-CLIENT-ID", mal_key)
        .send()
        .await
    {
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

    if resp.status() != 200 {
        return Err(format!(
            "Status code {}. The MAL api can't find this {}",
            resp.status(),
            search_type
        )
        .into());
    }

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
