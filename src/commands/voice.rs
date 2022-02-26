use crate::msgutils::*;
use crate::notifiers;
use crate::palette;

use serenity::{
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, misc::Mentionable},
};

use songbird::{Event, TrackEvent};

#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(
                msg.channel_id
                    .send_message(ctx, |m: &mut CreateMessage| {
                        create_embed_message(
                            m,
                            &String::from("Error"),
                            &String::from("You must be in a voice channel to use this command."),
                            palette::RED,
                            Some(msg),
                        );
                        m
                    })
                    .await,
            );

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .send_message(ctx, |m: &mut CreateMessage| {
                    create_embed_message(
                        m,
                        &String::from("Connected"),
                        &format!("Joined {}", connect_to.mention()),
                        palette::GREEN,
                        Some(msg),
                    );
                    m
                })
                .await,
        );

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        if !handle.is_deaf() {
            let _ = handle.deafen(true).await;
        }

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            notifiers::TrackEndNotifier {
                chan_id,
                http: send_http,
            },
        );
    } else {
        check_msg(msg.reply(&ctx.http, "Error joining the channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("fuckoff")]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        if msg.content.contains("fuckoff") {
            check_msg(
                msg.channel_id
                    .send_message(&ctx.http, |m: &mut CreateMessage| {
                        m.reference_message(msg);

                        m.embed(|e| {
                            e.title(":(");
                            e.description("nobody spanks t-bo!");
                            e.image("https://i.imgur.com/aofzYb8.png");
                            e
                        });

                        m
                    })
                    .await,
            );
            return Ok(())
        }

        success_react(ctx, msg).await;
    } else {
        success_react(ctx, msg).await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        if let Err(e) = handler.mute(false).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        success_react(ctx, msg).await;
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        success_react(ctx, msg).await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        if let Err(e) = handler.deafen(false).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        success_react(ctx, msg).await;
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        success_react(ctx, msg).await;
    }

    Ok(())
}