use crate::{
    notifiers,
    palette,
    utils::*,
    checks::*
};

use msgutils::*;

use serenity::{
    client::Context,
    framework::standard::{macros::command, macros::check, CommandResult},
    model::{channel::Message, misc::Mentionable},
};

use songbird::{Event, TrackEvent};

#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = match guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
    {
        Some(channel) => channel,
        None => {
            let message = generate_embed(
                String::from("Bruh"),
                String::from("Where tf are u"),
                palette::RED,
                Some(msg),
                None,
            );

            msgutils::send_handler(ctx, msg.channel_id, message).await;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, channel_id).await;

    if let Ok(_channel) = success {
        let mut handle = handle_lock.lock().await;

        if !handle.is_deaf() {
            let _ = handle.deafen(true).await;
        }

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            notifiers::TrackEndNotifier {
                chan_id: msg.channel_id,
                http: ctx.http.clone(),
                guild_id: guild_id,
                songbird_context: ctx.clone(),
            },
        );

        let message = generate_embed(
            String::from("Hi"),
            format!("What's up {}", channel_id.mention()),
            palette::GREEN,
            Some(msg),
            None,
        );

        msgutils::send_handler(ctx, msg.channel_id, message).await;
    } else {
        let message = generate_embed(
            String::from("Error"),
            String::from("Could not join voice channel"),
            palette::RED,
            Some(msg),
            None,
        );

        msgutils::send_handler(ctx, msg.channel_id, message).await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("fuckoff")]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    let handler = songbird::get(ctx)
        .await
        .expect("No songbird")
        .clone();

    if handler.get(guild_id).is_some() {
        if let Err(e) = handler.remove(guild_id).await {
            msgutils::check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        if msg.content.contains("fuckoff") {
            let message = generate_embed(
                String::from(":("),
                String::from("nobody spanks t-bo!"),
                palette::GREEN,
                Some(msg),
                Some("https://i.imgur.com/aofzYb8.png"),
            );

            msgutils::send_handler(ctx, msg.channel_id, message).await;
            return Ok(());
        }

        msgutils::success_react(ctx, msg).await;
    } else {
        // no handler, not in voice channel
    }

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

    let mut handler = call.lock().await;

    if handler.is_mute() {
        let _ = handler.mute(false).await;
        // tracing::info!("Unmuted");
        msgutils::success_react(ctx, msg).await;
    } else {
        let _ = handler.mute(true).await;
        // tracing::info!("muted");
        msgutils::success_react(ctx, msg).await;
    }

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

    let mut handler = call.lock().await;

    if handler.is_deaf() {
        let _ = handler.deafen(false).await;
        // tracing::info!("Unmuted");
        msgutils::success_react(ctx, msg).await;
    } else {
        let _ = handler.deafen(true).await;
        // tracing::info!("muted");
        msgutils::success_react(ctx, msg).await;
    }

    Ok(())
}
