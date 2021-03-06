use aspotify::Client as Spotify;
use dashmap::DashMap;
use futures::future::AbortHandle;
use lavalink_rs::LavalinkClient;
use reqwest::Client as Reqwest;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::{GuildId, UserId},
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

// All command context data structures
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

pub struct BotId;

impl TypeMapKey for BotId {
    type Value = UserId;
}

pub struct VoiceTimerMap;

impl TypeMapKey for VoiceTimerMap {
    type Value = Arc<DashMap<GuildId, AbortHandle>>;
}

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Reqwest;
}

pub struct SpotifyClient;

impl TypeMapKey for SpotifyClient {
    type Value = Arc<Spotify>;
}
