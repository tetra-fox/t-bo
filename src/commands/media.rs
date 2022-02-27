use std::time::Duration;

use crate::msgutils::*;
use crate::palette;

use super::voice;

use serenity::{
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::input::restartable::Restartable;
use songbird::EventContext::Track;

#[command]
#[aliases("play", "p", "bumpthis")]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .send_message(ctx, |m: &mut CreateMessage| {
                        create_embed_message(
                            m,
                            &String::from("Error"),
                            &String::from("Invalid syntax. Usage: `bo.queue <url|file>`"),
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

    if !url.starts_with("http") && !url.eq("file") {
        check_msg(
            msg.channel_id
                .send_message(ctx, |m: &mut CreateMessage| {
                    create_embed_message(
                        m,
                        &String::from("Error"),
                        &String::from("Invalid URL."),
                        palette::RED,
                        Some(msg),
                    );
                    m
                })
                .await,
        );

        return Ok(());
    }

    if url.eq("file") {
        url = match msg.attachments.first() {
            Some(attachment) => attachment.url.clone(),
            None => {
                check_msg(
                    msg.channel_id
                        .send_message(ctx, |m: &mut CreateMessage| {
                            create_embed_message(
                                m,
                                &String::from("Error"),
                                &String::from("No file attached."),
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
    }

    let guild = msg.guild(&ctx.cache).await.expect("No guild");
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    // if not in voice channel, join command runner's voice channel
    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            voice::join(ctx, msg, args).await?;
            let after_join_handler = match manager.get(guild_id) {
                Some(handler) => handler,
                None => {
                    return Ok(());
                }
            };
            after_join_handler
        }
    };

    let mut handler = handler_lock.lock().await;

    // Here, we use lazy restartable sources to make sure that we don't pay
    // for decoding, playback on tracks which aren't actually live yet.
    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);

            check_msg(
                msg.channel_id
                    .send_message(ctx, |m: &mut CreateMessage| {
                        create_embed_message(
                            m,
                            &String::from("Error"),
                            &String::from("Cannot get `ffmpeg`."),
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

    handler.enqueue_source(source.into());

    check_msg(
        msg.channel_id
            .send_message(ctx, |m: &mut CreateMessage| {
                create_embed_message(
                    m,
                    &String::from("I put food on sticks 😎"),
                    &format!(
                        "Now playing: {}.\nPosition on stick: {}",
                        handler.queue().len(),
                        handler.queue().len()
                    ),
                    palette::GREEN,
                    Some(msg),
                );
                m
            })
            .await,
    );

    Ok(())
}

#[command]
#[aliases("next")]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        check_msg(
            msg.channel_id
                .send_message(ctx, |m: &mut CreateMessage| {
                    create_embed_message(
                        m,
                        &String::from("Skipped"),
                        &format!("{} in queue.", queue.len()),
                        palette::YELLOW,
                        Some(msg),
                    );
                    m
                })
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .send_message(ctx, |m: &mut CreateMessage| {
                    create_embed_message(
                        m,
                        &String::from("Error"),
                        &String::from(
                            "You must first connect me to a voice channel with `bo.join`.",
                        ),
                        palette::RED,
                        Some(msg),
                    );
                    m
                })
                .await,
        );
    }

    Ok(())
}

#[command]
#[aliases("clear")]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.stop();

        check_msg(msg.reply(&ctx.http, "Queue cleared.").await);
    } else {
        check_msg(
            msg.channel_id
                .send_message(ctx, |m: &mut CreateMessage| {
                    create_embed_message(
                        m,
                        &String::from("Error"),
                        &String::from(
                            "You must first connect me to a voice channel with `bo.join`.",
                        ),
                        palette::RED,
                        Some(msg),
                    );
                    m
                })
                .await,
        );
    }

    Ok(())
}

#[command]
#[aliases("chill")]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if let Ok(_) = queue.pause() {
            success_react(ctx, msg).await;
        }
    };

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if let Ok(_) = queue.resume() {
            success_react(ctx, msg).await;
        }
    }

    Ok(())
}

#[command]
#[aliases("np", "current")]
#[only_in(guilds)]
async fn nowplaying(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        match queue.current() {
            Some(current_track) => {
                let default_unknown = &String::from("Unknown");
                let track_info = current_track.metadata();
                let track_title = track_info.title.as_ref().unwrap_or(default_unknown);
                let track_artist = track_info.artist.as_ref().unwrap_or(default_unknown);
                let track_length = track_info.duration.unwrap_or(Duration::new(0, 0));
                let track_position = Duration::new(0, 0);

                // format track length
                let track_length_str = match track_length.as_secs() {
                    0 => String::from("00:00"),
                    secs => {
                        let minutes = secs / 60;
                        let seconds = secs % 60;
                        format!("{:02}:{:02}", minutes, seconds)
                    }
                };

                // format track position
                let track_pos_str = match track_position.as_secs() {
                    0 => String::from("00:00"),
                    secs => {
                        let minutes = secs / 60;
                        let seconds = secs % 60;
                        format!("{:02}:{:02}", minutes, seconds)
                    }
                };

                check_msg(
                    msg.channel_id
                        .send_message(ctx, |m: &mut CreateMessage| {
                            create_embed_message(
                                m,
                                &String::from("Now playing"),
                                &format!(
                                    "**{}** - **{}**\n{}/{}",
                                    track_artist, track_title, track_pos_str, track_length_str
                                ),
                                palette::BLURPLE,
                                Some(msg),
                            );
                            m
                        })
                        .await,
                );
            }
            _ => {
                check_msg(
                    msg.channel_id
                        .send_message(ctx, |m: &mut CreateMessage| {
                            create_embed_message(
                                m,
                                &String::from("Now playing"),
                                &String::from("Nothing"),
                                palette::YELLOW,
                                Some(msg),
                            );
                            m
                        })
                        .await,
                );
            }
        }
    }

    Ok(())
}

#[command]
#[aliases("goto")]
#[only_in(guilds)]
async fn seek(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let (seek_time, seek_time_str): (Duration, String) = match args.single::<String>() {
        Ok(time) => {
            // parse string into duration
                
            let (mins, secs) = time.split_at(2);
            let toal_secs = mins.parse::<u64>().unwrap_or(0) * 60 + secs.parse::<u64>().unwrap_or(0);
            let time_duration = Duration::new(toal_secs, 0);

            (time_duration, time)
        }
        Err(_) => {
            check_msg(
                msg.channel_id
                    .send_message(ctx, |m: &mut CreateMessage| {
                        create_embed_message(
                            m,
                            &String::from("Error"),
                            &String::from("Invalid syntax. Use `bo.seek <time>` where `<time>` is format `mm:ss`."),
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

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        match queue.current() {
            Some(current_track) => {
                if current_track.is_seekable() {
                    let _ = current_track.seek_time(seek_time);
                }
                check_msg(
                    msg.channel_id
                        .send_message(ctx, |m: &mut CreateMessage| {
                            create_embed_message(
                                m,
                                &String::from("Seek"),
                                &format!("Seeked to {}", seek_time_str),
                                palette::BLURPLE,
                                Some(msg),
                            );
                            m
                        })
                        .await,
                );
            }
            _ => {
                check_msg(
                    msg.channel_id
                        .send_message(ctx, |m: &mut CreateMessage| {
                            create_embed_message(
                                m,
                                &String::from("Error"),
                                &String::from("Nothing is playing."),
                                palette::RED,
                                Some(msg),
                            );
                            m
                        })
                        .await,
                );
            }
        }
    }

    Ok(())
}
