use std::collections::HashSet;

use crate::structures::{commands::*, errors::*};
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError, DispatchError, StandardFramework},
    model::{channel::Message, id::UserId, Permissions},
};

pub fn get_framework(default_prefix: &str, owners: HashSet<UserId>) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix(default_prefix).owners(owners))
        .on_dispatch_error(dispatch_error)
        .group(&VOICE_GROUP)
        .group(&GENERAL_GROUP)
        .group(&MUSIC_GROUP)
        .group(&SUPPORT_GROUP)
}

// After a command is executed, goto here
#[hook]
async fn after(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        let part_1 = "Looks like the bot encountered an error! \n";
        let part_2 = "Please use the `support` command and send the output to the support server!";
        let error_string = format!("{}{}", part_1, part_2);

        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.color(0xff69b4);
                    e.title("Aw Snap!");
                    e.description(error_string);
                    e.field("Command Name", cmd_name, false);
                    e.field("Error", format!("```{} \n```", why), false);
                    e
                })
            })
            .await;
    }
}

// On a dispatch error, go to this function
#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    BardError::Permission(PermissionType::UserPerm("administrator")),
                )
                .await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    BardError::Permission(PermissionType::UserPerm("manage messages")),
                )
                .await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_EMOJIS) => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    BardError::Permission(PermissionType::UserPerm("manage emojis")),
                )
                .await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    format!("Args required: {}. Args given: {}", min, given),
                )
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(ctx, "This is a bot dev only command!")
                .await;
        }
        _ => println!("Unhandled dispatch error: {:?}", error),
    }
}
