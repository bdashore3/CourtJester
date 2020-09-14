use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command, Args
};
use crate::structures::cmd_data::{
    ReqwestClient,
    PubCreds
};
use reqwest::Url;
use serde::Deserialize;
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Response {
    results: Vec<GifResult>
}

#[derive(Debug, Deserialize)]
struct GifResult {
    url: String,
    media: Vec<HashMap<String, Media>>
}

#[derive(Debug, Deserialize)]
struct Media {
    url: String
}

#[command]
async fn hug(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() < 1 {
        msg.channel_id.say(ctx, "You want to give a hug? Please mention who you want to hug!").await?;
        return Ok(())
    }

    let gifs = fetch_gifs(ctx, "anime hug", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0xed9e2f);
            e.description(format!("{} hugs {}", msg.author.name, msg.mentions[0].name));
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

#[command]
async fn pat(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() < 1 {
        msg.channel_id.say(ctx, "I wanna pat someone! Please mention who to pat!").await?;
        return Ok(())
    }

    let gifs = fetch_gifs(ctx, "anime pat", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0x27e6d9);
            e.description(format!("{} pats {}", msg.author.name, msg.mentions[0].name));
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

#[command]
async fn slap(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() < 1 {
        msg.channel_id.say(ctx, "Wait... who do I slap again? Please mention the person!").await?;
        return Ok(())
    }

    let gifs = fetch_gifs(ctx, "anime slap", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0xd62929);
            e.description(format!("{} slaps {}", msg.author.name, msg.mentions[0].name));
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

#[command]
async fn cry(ctx: &Context, msg: &Message) -> CommandResult {
    let gifs = fetch_gifs(ctx, "anime cry", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0x3252e3);
            e.description(format!("{} is crying! 😭", msg.author.name));
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

#[command]
async fn cringe(ctx: &Context, msg: &Message) -> CommandResult {
    let gifs = fetch_gifs(ctx, "cringe", 10, "low").await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0x634644);
            e.description(format!("{} thinks that's really cringey 😬", msg.author.name));
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

#[command]
#[aliases("gif")]
async fn gifsearch(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() < 1 {
        msg.channel_id.say(ctx, "Please provide a search string after the command!").await?;
        return Ok(())
    }
    
    let search_string = args.message();

    let filter = 
        if msg.channel(ctx).await.unwrap().is_nsfw() {
            "off" 
        } else {
            "medium"
        };

    let gifs = fetch_gifs(ctx, search_string, 10, filter).await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0, 9);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.color(0x5ed13b);
            e.image(&gifs[val].media[0].get("gif").unwrap().url);
            e
        })
    }).await?;

    Ok(())
}

async fn fetch_gifs(ctx: &Context, search: &str, amount: u32, filter: &str) -> Result<Vec<GifResult>, Box<dyn std::error::Error + Send + Sync>> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let tenor_key = data.get::<PubCreds>().unwrap().get("tenor").unwrap().as_str();

    let url = Url::parse_with_params("https://api.tenor.com/v1/search",
            &[
                ("q", search),
                ("key", tenor_key),
                ("limit", &format!("{}", amount)),
                ("contentfilter", filter)
            ])?;

    let resp = reqwest_client.get(url)
        .send().await?
        .json::<Response>().await?;

    Ok(resp.results)
}

pub async fn image_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "gif: Fetches a random gif from tenor \nNote: The content filter is turned off in an NSFW channel \n\n",
        "hug <mention>: Gives wholesome hugs to someone \n\n",
        "pat <mention>: Pats someone on the head \n\n",
        "slap <mention>: Give someone a slap \n\n",
        "cry: Emphasizes that you're crying  \n\n",
        "cringe: Emphasizes that something is cringey \n\n");
    
    let _ = channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Images/Reaction Help");
            e.description("Description: Various commands that work with images");
            e.field("Commands", content, false);
            e
        })
    }).await;
}