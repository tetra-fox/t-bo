use crate::{checks::*, command_handlers, notifiers, palette, utils::*};

use msgutils::*;

use serenity::{
    client::Context,
    framework::standard::{macros::check, macros::command, Args, CommandResult, Delimiter},
    model::{channel::Message, misc::Mentionable},
};

use songbird::{Event, TrackEvent};
use url::Url;

/////////////////////////////////////////////////
/// Voice commands: join, leave, mute, deafen ///
/////////////////////////////////////////////////
#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    // get voice channel of message author
    let voice_channel_id = msg
        .guild(&ctx.cache)
        .await
        .expect("No guild")
        .voice_states
        .get(&msg.author.id)
        .unwrap()
        .channel_id
        .unwrap();

    // join channel
    let call =
        command_handlers::join(ctx, msg.guild_id.unwrap(), voice_channel_id, msg.channel_id).await;

    // inform channel of success
    let message = msgutils::generate_embed(
        String::from("Hi"),
        format!(
            "What's up {}. Notifications are bound to {}",
            voice_channel_id.mention(),
            msg.channel_id.mention()
        ),
        palette::GREEN,
        Some(msg),
        None,
    );

    msgutils::send_handler(ctx, msg.channel_id, message).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("fuckoff")]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    command_handlers::leave(ctx, msg.guild_id.unwrap()).await;

    // the funny
    if msg.content.contains("fuckoff") {
        let message = generate_embed(
            String::from(":("),
            String::from("nobody spanks t-bo!"),
            palette::FUCHSIA,
            Some(msg),
            Some("https://i.imgur.com/aofzYb8.png"),
        );

        msgutils::send_handler(ctx, msg.channel_id, message).await;
        return Ok(());
    }

    msgutils::success_react(ctx, msg).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    let call = songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call");

    command_handlers::mute(&call).await;

    msgutils::success_react(ctx, msg).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    let call = songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call");

    call.lock().await.queue();

    command_handlers::deafen(&call).await;

    msgutils::success_react(ctx, msg).await;

    Ok(())
}

/////////////////////////////////////////////////
/// Media commands: play, skip, stop, pause,  ///
/// resume, nowplaying, seek, queue, repeat   ///
/////////////////////////////////////////////////
#[command]
#[aliases("p", "bumpthis")]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    let call = match songbird::get(ctx).await.expect("No songbird").get(guild_id) {
        Some(call) => call,
        None => {
            let a = match mediautils::get_voice_channel_of_user(ctx, msg).await {
                Some(a) => a,
                None => {
                    let message = generate_embed(
                        String::from("Error"),
                        String::from("You are not in a voice channel"),
                        palette::RED,
                        Some(msg),
                        None,
                    );

                    msgutils::send_handler(ctx, msg.channel_id, message).await;
                    return Ok(());
                }
            };

            let after_join_call = command_handlers::join(
                ctx,
                msg.guild_id.unwrap(),
                a,
                msg.channel_id,
            )
            .await;

            after_join_call
        }
    };

    let first_arg = match args.single::<String>() {
        Ok(arg) => arg,
        Err(_) => {
            let mut message = generate_embed(
                String::from("Bruh"),
                String::from("U a stupid hoe. It go `bo.play <query|url|file>`"),
                palette::RED,
                Some(msg),
                None,
            );

            msg.channel_id.send_message(ctx, |_| &mut message).await;

            return Ok(());
        }
    };

    if first_arg.eq("file") || first_arg.eq("f") {
        let file_url = match msg.attachments.first() {
            Some(attachment) => attachment.url.clone(),
            None => {
                let mut message = msgutils::generate_embed(
                    String::from("???????????????????????"),
                    String::from("Where's the file tho"),
                    palette::RED,
                    Some(msg),
                    None,
                );

                msgutils::send_handler(ctx, msg.channel_id, message).await;

                return Ok(());
            }
        };

        command_handlers::play_url(&call, file_url, 0).await;
    } else {
        if let Ok(_) = Url::parse(&first_arg) {
            // if valid url play from url
            command_handlers::play_url(&call, first_arg, 0).await;
        } else {
            // fall back to youtube search
            let query = args.raw().collect::<Vec<&str>>().join(" ");
            command_handlers::play_search(&call, query, 0).await;
        }
    }

    let queue = call.lock().await.queue().current_queue();

    let mut message = generate_embed(
        String::from("I put food on sticks 😎"),
        format!(
            "Skewered **{}**\n Position on stick: `{}`",
            mediautils::construct_title_string(queue.last().unwrap().metadata()),
            call.lock().await.queue().len()
        ),
        palette::GREEN,
        Some(msg),
        None,
    );

    msg.channel_id.send_message(ctx, |_| &mut message).await;

    Ok(())
}
