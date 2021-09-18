use serenity::framework::standard::macros::group;

use crate::commands::{music::*, other::*, support::*, voice::*};

// All command groups

#[group]
#[help_available(false)]
#[commands(ping)]
pub struct General;

#[group("Support")]
#[description = "Support commands for the bot"]
#[commands(help, support)]
pub struct Support;

#[group("Voice")]
#[description = "Commands used for voice chat"]
#[commands(summon, disconnect)]
pub struct Voice;

#[group("Music")]
#[description = "Commands used to play music"]
#[commands(play, pause, resume, stop, skip, queue, clear, remove, seek)]
pub struct Music;
