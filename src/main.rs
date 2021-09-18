mod commands;
mod handlers;
mod helpers;
mod structures;

use std::{collections::HashSet, env, sync::Arc};

use aspotify::{Client as Spotify, ClientCredentials};
use dashmap::DashMap;
use futures::future::AbortHandle;
use lavalink_rs::LavalinkClient;
use reqwest::Client as Reqwest;
use serenity::{
    client::bridge::gateway::GatewayIntents, framework::standard::CommandResult, http::Http,
    model::id::GuildId, Client,
};
use structures::cmd_data::*;

use songbird::SerenityInit;

use crate::handlers::{
    event_handler::{LavalinkHandler, SerenityHandler},
    framework::get_framework,
};

#[tokio::main]
async fn main() -> CommandResult {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials::read_creds(&args[1])?;

    let http = Http::new_with_token(&creds.bot_token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let voice_timer_map: DashMap<GuildId, AbortHandle> = DashMap::new();

    let lava_client = LavalinkClient::builder(bot_id)
        .set_host(creds.lavalink_host)
        .set_password(creds.lavalink_auth)
        .build(LavalinkHandler)
        .await?;

    let client_credentials = ClientCredentials {
        id: creds.spotify_client_id,
        secret: creds.spotify_client_secret,
    };

    let spotify = Spotify::new(client_credentials);

    let reqwest_client = Reqwest::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0")
        .build()?;

    let mut client = Client::builder(&creds.bot_token)
        .framework(get_framework(&creds.default_prefix, owners))
        .event_handler(SerenityHandler)
        .intents({
            GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MESSAGES
        })
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        // Insert all structures into ctx data
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<Lavalink>(lava_client);
        data.insert::<BotId>(bot_id);
        data.insert::<VoiceTimerMap>(Arc::new(voice_timer_map));
        data.insert::<ReqwestClient>(reqwest_client);
        data.insert::<SpotifyClient>(Arc::new(spotify));
    }

    // Start up the bot! If there's an error, let the user know
    if let Err(why) = client.start_autosharded().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
