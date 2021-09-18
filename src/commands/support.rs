use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::{
    commands::{music::music_help, voice::voice_help},
};

#[command]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        default_help_message(ctx, msg.channel_id).await;

        return Ok(());
    }

    let subcommand = args.single::<String>()?;

    match subcommand.as_str() {
        "voice" => voice_help(ctx, msg.channel_id).await,
        "music" => music_help(ctx, msg.channel_id).await,
        _ => {}
    }

    Ok(())
}

async fn default_help_message(ctx: &Context, channel_id: ChannelId) {
    let categories = concat!(
        "voice - For general voice commands\n",
        "music - For all music commands\n"
    );

    let _ = channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("CourtJester Help");
                e.description(concat!(
                    "Help for the Bard Discord bot \n",
                    "Command parameters: <> is required and () is optional \n",
                    "Please use `help <subcategory>` to see that category's help"
                ));
                e.field("Subcategories", format!("```\n{}```", categories), false);
                e.footer(|f| {
                    f.text("Use the support command for any further help!");
                    f
                });
                e
            })
        })
        .await;
}

#[command]
async fn support(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Bard Support");
                e.description("Need more help?");
                e.field("Support Server", "https://discord.gg/pswt7by", false);
                e.field(
                    "Github repository",
                    "https://github.com/bdashore3/bard",
                    false,
                );
                e.field("kingbri's socials", "https://kingbri.dev/socials", false);
                e.footer(|f| {
                    f.text("Created with ❤️ by kingbri#6666.\n Consider giving a GitHub star!");
                    f
                })
            })
        })
        .await?;

    Ok(())
}
