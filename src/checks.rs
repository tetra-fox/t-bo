use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::prelude::*,
    prelude::Mentionable,
};
use tracing::{span, Instrument, Level};

use crate::utils::msgutils::{self};
use crate::palette::{self};

#[check]
#[name = "in_voice"]
// Ensures a command is only usable if in the same voice channel as sunny
pub async fn in_same_voice_check(
    ctx: &Context,
    msg: &Message,
    _args: &mut Args,
    _command_options: &CommandOptions,
) -> Result<(), Reason> {
    let span = span!(Level::INFO, "in_same_voice_check", ?msg);
    tracing::info!("checking");
    async move {
        let songbird = songbird::get(ctx)
            .await
            .expect("Failed to get songbird");

        let guild_id = msg
            .guild_id
            .expect("Guild ID Empty");

        let channel = {
            // bot is not in a call
            let songbird_call_m = songbird
                .get(guild_id)
                .expect("Bot is not currently in a call");

            let songbird_call = songbird_call_m.lock().await;

            // could not find channel for whatever reason
            songbird_call
                .current_channel()
                .expect("Couldn't find songbird channel")
        };

        let guild = msg
            .guild(&ctx.cache)
            .await
            .expect("Couldn't get guild");

        let mut states = guild.voice_states.values();

        states
            .any(|vs| match vs.channel_id {
                Some(c_id) => channel.0 == c_id.0 && vs.user_id.0 == msg.author.id.0,
                None => false,
            })
            .then(|| ())
            .ok_or_else(async || {
                tracing::info!("ah fuck");
                let message = msgutils::generate_embed(
                    String::from("Error"),
                    format!("You must be in {} to use commands", channel.0),
                    palette::RED,
                    Some(msg),
                    None,
                );
                msgutils::send_handler(ctx, msg.channel_id, message).await;
            });
        Ok(())
    }
    .instrument(span)
    .await
}
