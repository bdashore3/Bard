use serde::{Deserialize, Serialize};
use serenity::framework::standard::CommandResult;
use std::{fs, io::BufReader};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub bot_token: String,
    pub default_prefix: String,
    pub lavalink_host: String,
    pub lavalink_auth: String,
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
}

pub fn read_creds(path: &str) -> CommandResult<Credentials> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let info: Credentials = serde_json::from_reader(reader).unwrap();

    Ok(info)
}
