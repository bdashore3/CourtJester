use std::{
    env,
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        StandardFramework,
        CommandResult,
        CommandGroup,
        CommandError,
        DispatchError,
        HelpOptions,
        help_commands,
        Args,
        macros:: {
            group,
            help,
            hook
        }
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready, id::GuildId},
    model::prelude:: {
        Permissions,
        UserId,
        Message
    },
    
    prelude::*
};

use commands::{
    other::*,
    textmod::*,
    ciphers::*,
    textchannel_send::*,
    config::*
};

use sqlx::PgPool;
use dashmap::DashMap;

use helpers:: {
    database_helper,
    prefix_cache
};

mod commands;
mod helpers;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

struct PrefixMap;

impl TypeMapKey for PrefixMap { 
    type Value = Arc<DashMap<i64, String>>;
}

struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = Arc<String>;
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials_helper::read_creds(args[1].to_string()).unwrap();
    let token = &creds.bot_token;

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    #[hook]
    async fn unrecognized_command_hook(ctx: &Context, msg: &Message, command_name: &str) {
        let data = ctx.data.read().await;

        let pool = data.get::<ConnectionPool>().unwrap();
    
        let guild_id = msg.guild_id.unwrap();

        let command = sqlx::query!("SELECT * FROM commands WHERE guild_id = $1 AND name = $2", guild_id.0 as i64, command_name)
            .fetch_optional(pool)
            .await
            .unwrap();
        
        if let Some(x) = command {
            let _ = msg.channel_id.say(ctx, format!("{}", x.content.unwrap())).await;
        }
    }

    #[hook]
    async fn after(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
        if let Err(why) = error {
            println!("Error in {}: {:?}", cmd_name, why);
        }
    }

    #[hook]
    async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
        match error {
            DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
                let _ = msg.channel_id.say(ctx, "You can't execute this command because you aren't an administrator!");
            },
            DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
                let _ = msg.channel_id.say(ctx, "You can't exeucte this command because you aren't a moderator (Manage Messages permission)!");
            },
            DispatchError::NotEnoughArguments { min, given } => {
                let _ = msg.channel_id.say(ctx, format!("Args required: {}. Args given: {}", min, given)).await;
            },
            _ => println!("Unhandled dispatch error"),
        }        
    }

    #[hook]
    async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
        let prefix;

        let data = ctx.data.read().await;
        let default_prefix = data.get::<DefaultPrefix>().unwrap();

        if let Some(id) = msg.guild_id {

            let prefixes = data.get::<PrefixMap>().unwrap();

            let guild_id = msg.guild_id.unwrap().0 as i64;

            prefix = match prefixes.get(&guild_id) {
                Some(prefix) => prefix.to_string(),
                None => default_prefix.to_string(),
            };
        }
        else {
            prefix = default_prefix.to_string();
        }
        Some(prefix)
    }

    #[help]
    #[individual_command_tip = "Hi there! \n
    This is the help for all the bot's commands! Just pass the command/category name as an argument! \n"]
    #[lacking_permissions = "Hide"]
    async fn send_help(
        ctx: &Context,
        msg: &Message,
        args: Args,
        help_options: &'static HelpOptions,
        groups: &[&'static CommandGroup],
        owners: HashSet<UserId>
    ) -> CommandResult {
        help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await
    }

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
        .help(&SEND_HELP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        let pool = database_helper::obtain_pool(creds.db_connection).await?;
        data.insert::<ConnectionPool>(pool.clone());

        let prefixes = prefix_cache::fetch_prefixes(&pool).await?;
        data.insert::<PrefixMap>(Arc::new(prefixes));

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));

        data.insert::<DefaultPrefix>(Arc::new(creds.default_prefix));
    }

    let _owners = match client.cache_and_http.http.get_current_application_info().await {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
