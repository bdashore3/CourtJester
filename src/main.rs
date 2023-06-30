mod commands;
mod handlers;
mod helpers;
mod reactions;
mod structures;

use crate::{
    handlers::{event_handler::SerenityHandler, framework::get_framework},
    helpers::{command_utils, database_helper},
    structures::{cmd_data::*, commands::*, errors::*},
};
// use aspotify::{Client as Spotify, ClientCredentials};
use dashmap::DashMap;
use futures::future::AbortHandle;
// use lavalink_rs::LavalinkClient;
use reqwest::Client as Reqwest;
use serenity::{framework::standard::CommandResult, http::Http, model::id::GuildId, prelude::*};
// use songbird::SerenityInit;
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{atomic::AtomicBool, Arc},
};

#[tokio::main]
async fn main() -> CommandResult {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials_helper::read_creds(&args[1])?;
    let token = &creds.bot_token;

    let http = Http::new_with_application_id(&token, creds.application_id);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let pool = database_helper::obtain_db_pool(creds.db_connection).await?;
    let prefixes = database_helper::fetch_prefixes(&pool).await?;
    let voice_timer_map: DashMap<GuildId, AbortHandle> = DashMap::new();

    /*
    let lava_client = LavalinkClient::builder(bot_id)
        .set_host(creds.lavalink_host)
        .set_password(creds.lavalink_auth)
        .build(LavalinkHandler)
        .await?;
    */

    let mut pub_creds = HashMap::new();
    pub_creds.insert("tenor".to_string(), creds.tenor_key);
    pub_creds.insert("default prefix".to_string(), creds.default_prefix);
    pub_creds.insert("mal".to_string(), creds.mal_key);

    /*
    let client_credentials = ClientCredentials {
        id: creds.spotify_client_id,
        secret: creds.spotify_client_secret,
    };

    let spotify = Spotify::new(client_credentials);
    */

    let emergency_commands = command_utils::get_allowed_commands();

    let command_names = MASTER_GROUP
        .options
        .sub_groups
        .iter()
        .flat_map(|x| {
            x.options
                .commands
                .iter()
                .flat_map(|i| i.options.names.iter().map(ToString::to_string))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<String>>();

    let reqwest_client = Reqwest::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0")
        .build()?;

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::DIRECT_MESSAGES);
    intents.remove(GatewayIntents::DIRECT_MESSAGE_REACTIONS);
    intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);

    let mut client = Client::builder(&token, intents)
        .framework(get_framework(bot_id, owners))
        .event_handler(SerenityHandler {
            run_loop: AtomicBool::new(true),
        })
        //.register_songbird()
        .await
        .expect("Err creating client");

    {
        // Insert all structures into ctx data
        let mut data = client.data.write().await;

        data.insert::<ConnectionPool>(pool.clone());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        // data.insert::<Lavalink>(lava_client);
        data.insert::<VoiceTimerMap>(Arc::new(voice_timer_map));
        data.insert::<PrefixMap>(Arc::new(prefixes));
        data.insert::<CommandNameMap>(Arc::new(command_names));
        data.insert::<ReqwestClient>(reqwest_client);
        data.insert::<PubCreds>(Arc::new(pub_creds));
        data.insert::<EmergencyCommands>(Arc::new(emergency_commands));
        data.insert::<BotId>(bot_id);
        // data.insert::<SpotifyClient>(Arc::new(spotify));
        data.insert::<ReactionImageCache>(Arc::new(DashMap::new()));
    }

    // Start up the bot! If there's an error, let the user know
    if let Err(why) = client.start_autosharded().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
