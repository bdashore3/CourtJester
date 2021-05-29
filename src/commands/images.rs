use crate::helpers::reaction_gifs::{add_to_cache, check_image_cache, fetch_gifs};
use rand::{prelude::StdRng, Rng, SeedableRng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let is_everyone = match args.single::<String>() {
        Ok(test) => &test == "everyone" || &test == "Everyone",
        Err(_) => false,
    };

    if msg.mentions.is_empty() && !is_everyone {
        msg.channel_id
            .say(
                ctx,
                "You want to give a hug? Please mention who you want to hug or provide `everyone`!",
            )
            .await?;
        return Ok(());
    }

    let raw_gifs = fetch_gifs(ctx, "anime hug", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "hug".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    let message = if is_everyone {
        "Group hug!".to_owned()
    } else if msg.mentions[0].id == msg.author.id {
        "You hugged yourself. Cute 🙂".to_owned()
    } else {
        format!("{} hugs {}", msg.author.name, msg.mentions[0].name)
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0xed9e2f);
                e.description(message);
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "hug".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
async fn pat(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let is_everyone = match args.single::<String>() {
        Ok(test) => (&test == "everyone" || &test == "Everyone"),
        Err(_) => false,
    };

    if msg.mentions.is_empty() && !is_everyone {
        msg.channel_id
            .say(
                ctx,
                "I wanna pat someone! Please mention who to pat or provide `everyone`!",
            )
            .await?;

        return Ok(());
    }

    let raw_gifs = fetch_gifs(ctx, "anime pat", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "pat".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    let message = if is_everyone {
        "Pats for everyone!".to_owned()
    } else if msg.mentions[0].id == msg.author.id {
        "You gave yourself a pat on the back!".to_owned()
    } else {
        format!("{} pats {}", msg.author.name, msg.mentions[0].name)
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x27e6d9);
                e.description(message);
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "pat".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
async fn slap(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let is_everyone = match args.single::<String>() {
        Ok(test) => (&test == "everyone" || &test == "Everyone"),
        Err(_) => false,
    };

    if msg.mentions.is_empty() && !is_everyone {
        msg.channel_id
            .say(
                ctx,
                "Wait... who do I slap again? Please mention the person or provide `everyone`!",
            )
            .await?;

        return Ok(());
    }

    let raw_gifs = fetch_gifs(ctx, "anime slap", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "slap".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    let message = if is_everyone {
        "You slapped everyone! Ouch... that's gotta hurt.".to_owned()
    } else if msg.mentions[0].id == msg.author.id {
        "You slapped yourself? Not sure if that's a good or bad thing...".to_owned()
    } else {
        format!("{} slaps {}", msg.author.name, msg.mentions[0].name)
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0xd62929);
                e.description(message);
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "slap".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
async fn kiss(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let is_everyone = match args.single::<String>() {
        Ok(test) => &test == "everyone" || &test == "Everyone",
        Err(_) => false,
    };

    if msg.mentions.is_empty() && !is_everyone {
        msg.channel_id
            .say(
                ctx,
                "You want to express your feelings? Please mention who you want to kiss or provide `everyone`!",
            )
            .await?;
        return Ok(());
    }

    let raw_gifs = fetch_gifs(ctx, "anime kiss", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "kiss".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    let message = if is_everyone {
        "A friendly kiss to everyone!".to_owned()
    } else if msg.mentions[0].id == msg.author.id {
        "Well... You just kissed yourself".to_owned()
    } else {
        format!("{} kisses {}", msg.author.name, msg.mentions[0].name)
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0xffb6c1);
                e.description(message);
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "kiss".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
async fn disgust(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_gifs = fetch_gifs(ctx, "anime disgust", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "disgust".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x50c878);
                e.description(format!("{} is disgusted 😕", msg.author.name));
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(
        ctx,
        guild_id,
        "disgust".to_owned(),
        gifs[val].url.to_owned(),
    )
    .await;

    Ok(())
}

#[command]
async fn cry(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_gifs = fetch_gifs(ctx, "anime cry", 10, "medium").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "cry".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x3252e3);
                e.description(format!("{} is crying! 😭", msg.author.name));
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "cry".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
async fn cringe(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_gifs = fetch_gifs(ctx, "cringe", 10, "low").await?;
    let mut rng = StdRng::from_entropy();

    let guild_id = msg.guild_id.unwrap();
    let gifs = check_image_cache(ctx, guild_id, "cringe".to_owned(), raw_gifs).await;

    let val = rng.gen_range(0..=gifs.len() - 1);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x634644);
                e.description(format!(
                    "{} thinks that's really cringey 😬",
                    msg.author.name
                ));
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    add_to_cache(ctx, guild_id, "cringe".to_owned(), gifs[val].url.to_owned()).await;

    Ok(())
}

#[command]
#[aliases("gif")]
async fn gifsearch(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .say(ctx, "Please provide a search string after the command!")
            .await?;
        return Ok(());
    }

    let search_string = args.message();

    let filter = if msg.channel(ctx).await.unwrap().is_nsfw() {
        "off"
    } else {
        "medium"
    };

    let gifs = fetch_gifs(ctx, search_string, 10, filter).await?;
    let mut rng = StdRng::from_entropy();
    let val = rng.gen_range(0..=gifs.len() - 1);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(0x5ed13b);
                e.image(&gifs[val].media[0].get("gif").unwrap().url);
                e
            })
        })
        .await?;

    Ok(())
}

pub async fn image_help(ctx: &Context, channel_id: ChannelId) {
    let content = concat!(
        "gif: Fetches a random gif from tenor \nNote: The content filter is turned off in an NSFW channel \n\n",
        "hug <mention>: Gives wholesome hugs to someone \n\n",
        "pat <mention>: Pats someone on the head \n\n",
        "slap <mention>: Give someone a slap \n\n",
        "kiss <mention>: You already know what this is and I am shaking my head... \n\n",
        "cry: Emphasizes that you're crying  \n\n",
        "cringe: Emphasizes that something is cringey \n\n");

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Images/Reaction Help");
                e.description("Description: Various commands that work with images");
                e.field("Commands", content, false);
                e
            })
        })
        .await;
}
