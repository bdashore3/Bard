use lavalink_rs::gateway::LavalinkEventHandler;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::Ready,
};

pub struct SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

pub struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}
