mod commands;
mod helpers;
mod structures;
mod reactions;

use std::{
    env,
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    async_trait,
    framework::standard::{
        StandardFramework,
        CommandError,
        DispatchError,
        macros:: {
            group,
            hook
        }
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready, guild::Guild, guild::PartialGuild, channel::Reaction, id::GuildId},
    model::prelude:: {
        Permissions,
        Message
    },
    prelude::*
};

use commands::{
    other::*,
    textmod::*,
    ciphers::*,
    textchannel_send::*,
    config::*,
    support::*,
    starboard::*,
    music::*
};

use structures::*;
use helpers::database_helper;
use helpers::voice_utils::*;
use reactions::reaction_handler;
use serenity_lavalink::LavalinkClient;
use futures::future::AbortHandle;
use dashmap::DashMap;

// All command groups
#[group]
#[help_available(false)]
#[commands(ping)]
struct General;

#[group("Text Modification")]
#[description = "Commands than modify text. \n
Append l in the command to use the last message \n
Example: `mockl` mocks the last message"]
#[commands(mock, inv, upp, low, space, biggspace)]
struct Text;

#[group]
#[help_available(false)]
#[commands(mockl, invl, uppl, lowl, spacel, biggspacel)]
struct TextLast;

#[group("Ciphers")]
#[description = "Commands that encode/decode messages"]
#[commands(b64encode, b64decode)]
struct Ciphers;

#[group("Jars")]
#[description = "Commands that send certain messages to channels"]
#[commands(nice, bruh, quote)]
struct TextChannelSend;

#[group("Bot Configuration")]
#[description = "Admin/Moderator commands that configure the bot"]
#[commands(prefix, command)]
struct Config;

#[group("Support")]
#[description = "Support commands for the bot"]
#[commands(help)]
struct Support;

#[group("Starboard")]
#[description = "Starboard admin commands"]
#[commands(starboard)]
struct Starboard;

#[group("Voice")]
#[description = "Commands used for voice chat"]
#[commands(summon, disconnect)]
struct Voice;

#[group("Music")]
#[description = "Commands used to play music"]
#[commands(play, pause, resume, now_playing, queue, skip, stop, clear)]
struct Music;

// Event handler for when the bot starts
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {

        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let guild_id = guild.id.0 as i64;

        if is_new {
            sqlx::query!("INSERT INTO guild_info VALUES($1, null) ON CONFLICT DO NOTHING", guild_id)
                .execute(pool).await.unwrap();
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: PartialGuild, _full: Option<Guild>) {
        
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let guild_id = incomplete.id.0 as i64;

        sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild_id)
            .execute(pool).await.unwrap();        
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let _ = reaction_handler::dispatch_reaction(&ctx, &reaction, false).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let _ = reaction_handler::dispatch_reaction(&ctx, &reaction, true).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials_helper::read_creds(args[1].to_string()).unwrap();
    let token = &creds.bot_token;

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let pool = database_helper::obtain_pool(creds.db_connection).await?;
    let voice_timer_map: DashMap<GuildId, AbortHandle> = DashMap::new(); 

    let mut lava_client = LavalinkClient::new();
    lava_client.password = creds.lavalink_auth;
    lava_client.bot_id = bot_id;
    lava_client.initialize().await?;

    // If there's no command, check in the custom commands database
    #[hook]
    async fn unrecognized_command_hook(ctx: &Context, msg: &Message, command_name: &str) {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let guild_id = msg.guild_id.unwrap().0 as i64;

        let cmd_data = sqlx::query!(
                "SELECT content FROM commands WHERE guild_id = $1 AND name = $2",
                guild_id, command_name)
            .fetch_optional(pool).await.unwrap();

        if let Some(cmd_data) = cmd_data {
            let content = cmd_data.content.unwrap()
                .replace("{user}", &msg.author.mention());
            let _ = msg.channel_id.say(ctx, content).await;
        }
    }

    // After a command is executed, goto here
    #[hook]
    async fn after(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
        if let Err(why) = error {
            println!("Error in {}: {:?}", cmd_name, why);
        }
    }

    // On a dispatch error, go to this function
    #[hook]
    async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
        match error {
            DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
                let _ = msg.channel_id.say(ctx, 
                    "You can't execute this command because you aren't an administrator!").await;
            },
            DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
                let _ = msg.channel_id.say(ctx, 
                    "You can't execute this command because you aren't a moderator! (Manage Messages permission)").await;
            },
            DispatchError::NotEnoughArguments { min, given } => {
                let _ = msg.channel_id.say(ctx, format!("Args required: {}. Args given: {}", min, given)).await;
            },
            DispatchError::OnlyForOwners => {
                let _ = msg.channel_id.say(ctx, "This is a bot dev only command!").await;
            }
            _ => println!("Unhandled dispatch error: {:?}", error),
        }        
    }

    /*
     * The heart of custom prefixes
     * If the guild has a prefix in the Dashmap, use that prefix
     * Otherwise, use the default prefix from credentials_helper
     */
    #[hook]
    async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let default_prefix = data.get::<DefaultPrefix>().unwrap();
        let guild_id = msg.guild_id.unwrap();

        let cur_prefix = commands::config::get_prefix(pool, guild_id, default_prefix.to_string()).await.unwrap();

        Some(cur_prefix)
    }

    // Link everything together!
    let framework = StandardFramework::new()
        .configure(|c| c
            .dynamic_prefix(dynamic_prefix)
            .owners(owners)
        )

        .on_dispatch_error(dispatch_error)
        .unrecognised_command(unrecognized_command_hook)
        .after(after)
        
        .group(&GENERAL_GROUP)
        .group(&TEXT_GROUP)
        .group(&TEXTLAST_GROUP)
        .group(&CIPHERS_GROUP)
        .group(&TEXTCHANNELSEND_GROUP)
        .group(&CONFIG_GROUP)
        .group(&SUPPORT_GROUP)
        .group(&STARBOARD_GROUP)
        .group(&VOICE_GROUP)
        .group(&MUSIC_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        // Insert all structures into ctx data
        let mut data = client.data.write().await;

        data.insert::<ConnectionPool>(pool.clone());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<DefaultPrefix>(Arc::new(creds.default_prefix));
        data.insert::<Lavalink>(Arc::new(RwLock::new(lava_client)));
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<VoiceTimerMap>(Arc::new(voice_timer_map));
    }

    let _owners = match client.cache_and_http.http.get_current_application_info().await {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    // Start up the bot! If there's an error, let the user know
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
