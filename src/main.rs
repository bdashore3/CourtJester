mod commands;
mod helpers;

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
    model::{event::ResumedEvent, gateway::Ready, guild::Guild, guild::PartialGuild},
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
    guild_cache,
    guild_cache::GuildData
};

// All command context data structures
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

struct GuildMap;

impl TypeMapKey for GuildMap { 
    type Value = Arc<DashMap<i64, GuildData>>;
}

struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = Arc<String>;
}

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
#[commands(prefix, command, restore)]
struct Config;

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
        let guild_map = data.get::<GuildMap>().unwrap();

        if is_new {
            sqlx::query!("INSERT INTO guild_info VALUES($1, null)", guild_id)
                .execute(pool).await.unwrap();
            
            let new_guild_data = GuildData {
                prefix: String::from(""),
                commands: DashMap::new()
            };

            guild_map.insert(guild_id, new_guild_data);
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: PartialGuild, _full: Option<Guild>) {
        
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let guild_id = incomplete.id.0 as i64;
        let guild_map = data.get::<GuildMap>().unwrap();

        sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild_id)
            .execute(pool).await.unwrap();
        
        guild_map.remove(&guild_id);
    }
}

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

    // If there's no command, check in the custom commands database
    #[hook]
    async fn unrecognized_command_hook(ctx: &Context, msg: &Message, command_name: &str) {
        let data = ctx.data.read().await;

        let pool = data.get::<ConnectionPool>().unwrap();
    
        let guild_id = msg.guild_id.unwrap().0 as i64;
        let guild_map = data.get::<GuildMap>().unwrap();
        let guild_data = guild_map.get(&guild_id).unwrap();

        if !guild_data.commands.contains_key(command_name) {
            return
        }

        let command_content = guild_data.commands.get(command_name).unwrap();
        let _ = msg.channel_id.say(ctx, format!("{}", *command_content)).await;
        
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
        let prefix;

        let data = ctx.data.read().await;
        let default_prefix = data.get::<DefaultPrefix>().unwrap();

        if let Some(id) = msg.guild_id {

            let guild_map = data.get::<GuildMap>().unwrap();
            let guild_id = msg.guild_id.unwrap().0 as i64;
            let guild_data = guild_map.get(&guild_id).unwrap();

            prefix = match guild_data.prefix.as_str() {
                "" => default_prefix.to_string(),
                _ => guild_data.prefix.to_string()
            };
        }
        else {
            prefix = default_prefix.to_string();
        }
        Some(prefix)
    }

    // Heart of the help command system
    #[help]
    #[individual_command_tip = "Hi there! \n
    This is the help for all the bot's commands! Just pass the command/category name as an argument! \n"]
    #[lacking_permissions = "Hide"]
    #[lacking_ownership = "Hide"]
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
        .help(&SEND_HELP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        // Insert all structures into ctx data
        let mut data = client.data.write().await;

        let pool = database_helper::obtain_pool(creds.db_connection).await?;
        data.insert::<ConnectionPool>(pool.clone());

        let guild_data = guild_cache::fetch_guild_data(&pool).await?;

        data.insert::<GuildMap>(Arc::new(guild_data));

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

    // Start up the bot! If there's an error, let the user know
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
