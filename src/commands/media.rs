#![allow(unused_must_use)]
use std::time::Duration;

use crate::{
    palette,
    utils::{msgutils::*, *},
};

use serenity::framework::standard::Delimiter;
use songbird::tracks::LoopState;
use url::Url;

use super::voice;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::input::restartable::Restartable;

// big boy handles urls, files, and youtube search
#[command]
#[aliases("p", "bumpthis")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let mut url = String::default();

    if first_arg.eq("file") {
        url = match msg.attachments.first() {
            Some(attachment) => attachment.url.clone(),
            None => {
                let mut message = msgutils::generate_embed(
                    String::from("???????????????????????"),
                    String::from("Where's the file tho"),
                    palette::RED,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;

                return Ok(());
            }
        };
    }

    // if not in voice channel, join command runner's voice channel
    let handler_lock =
        match mediautils::get_songbird(ctx, msg.guild(&ctx.cache).await.expect("No guil").id).await
        {
            Some(handler) => handler,
            None => {
                type Arfs = Args;
                voice::join(ctx, msg, Arfs::new("", &[Delimiter::Single(' ')])).await?;
                let after_join_handler = match mediautils::get_songbird(
                    ctx,
                    msg.guild(&ctx.cache).await.expect("No guil :(").id,
                )
                .await
                {
                    Some(handler) => handler,
                    None => {
                        return Ok(());
                    }
                };
                after_join_handler
            }
        };

    let mut handler = handler_lock.lock().await;

    let valid_url = match Url::parse(&first_arg) {
        Ok(_) => {
            url = first_arg;
            true
        }
        Err(_) => false,
    };

    let source;
    if valid_url {
        source = match Restartable::ytdl(url, true).await {
            Ok(owo) => owo,
            Err(uwu) => {
                println!("This shit broke: {:?}", uwu);

                let mut message = generate_embed(
                    String::from("Yo"),
                    String::from("I can't play that"),
                    palette::RED,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;

                return Ok(());
            }
        };
    } else {
        let query = args.raw().collect::<Vec<&str>>().join(" ");
        source = match Restartable::ytdl_search(query, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                let mut message = generate_embed(
                    String::from("Why'd u send a jpg"),
                    String::from("That's a jpg bro"),
                    palette::RED,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;

                return Ok(());
            }
        };
    }

    handler.enqueue_source(source.into());

    let mut message = generate_embed(
        String::from("I put food on sticks 😎"),
        format!(
            "Skewered **{}**\n Position on stick: `{}`",
            mediautils::construct_title_string(
                &handler.queue().current_queue().last().unwrap().metadata()
            ),
            handler.queue().len()
        ),
        palette::GREEN,
        Some(msg),
        None,
    );

    msg.channel_id.send_message(ctx, |_| &mut message).await;

    Ok(())
}

#[command]
#[aliases("s", "next")]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guil").id;

    let queue = songbird::get(ctx)
        .await
        .expect("No songbir")
        .get(guild_id)
        .expect("Coulnt get bird khaulle")
        .lock()
        .await
        .queue()
        .clone();

    if let Ok(_) = queue.skip() {
        let mut message = generate_embed(
            String::from("Skipp"),
            format!("podition on sticc: {}", queue.len()),
            palette::GREEN,
            Some(msg),
            None,
        );

        msg.channel_id.send_message(ctx, |_| &mut message).await;
    }

    Ok(())
}

#[command]
#[aliases("clear")]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call")
        .lock()
        .await
        .queue()
        .stop();

    msgutils::success_react(ctx, msg).await;

    Ok(())
}

#[command]
#[aliases("chill")]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call")
        .lock()
        .await
        .queue()
        .pause()
        .map_err(|_| {
            tracing::error!("Cannot pause");
        });

    msgutils::success_react(ctx, msg);

    Ok(())
}

#[command]
#[only_in(guilds)]
// #[checks(in_voice)]
async fn resume(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    match songbird::get(ctx).await.expect("No songbird").get(guild_id) {
        Some(call) => {
            call.lock().await.queue().resume();
            msgutils::success_react(ctx, msg).await;
        }
        None => {
            let mut message = generate_embed(
                String::from("??????????????????????????????????????"),
                String::from("ᵂʰᵃᵗ"),
                palette::RED,
                Some(msg),
                None,
            );

            msg.channel_id.send_message(ctx, |_| &mut message).await;

            return Ok(());
        }
    }

    Ok(())
}

#[command]
#[aliases("np", "current")]
#[only_in(guilds)]
async fn nowplaying(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Some(handler_lock) =
        mediautils::get_songbird(ctx, msg.guild(&ctx.cache).await.expect("No guild").id).await
    {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        match queue.current() {
            Some(current_track) => {
                let msg_content = mediautils::construct_np_msg(&current_track).await;

                let mut message = generate_embed(
                    String::from("We bumpin'"),
                    msg_content,
                    palette::BLURPLE,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;
            }
            _ => {
                let mut message = generate_embed(
                    String::from("We are not bumpin'"),
                    String::from("No more choons :("),
                    palette::YELLOW,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;
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
            let toal_secs =
                mins.parse::<u64>().unwrap_or(0) * 60 + secs.parse::<u64>().unwrap_or(0);
            let time_duration = Duration::new(toal_secs, 0);

            (time_duration, time)
        }
        Err(_) => {
            let mut message = generate_embed(
                String::from("Bruh"),
                String::from(
                    "Invalid syntax. Use `bo.seek <time>` where `<time>` is format `mm:ss`.",
                ),
                palette::BLURPLE,
                Some(msg),
                None,
            );

            msg.channel_id.send_message(ctx, |_| &mut message).await;

            return Ok(());
        }
    };

    if let Some(handler_lock) =
        mediautils::get_songbird(ctx, msg.guild(&ctx.cache).await.unwrap().id).await
    {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        match queue.current() {
            Some(current_track) => {
                if current_track.is_seekable() {
                    let _ = current_track.seek_time(seek_time);
                }

                let mut message = generate_embed(
                    String::from("Seek"),
                    format!("Seeked to {}", seek_time_str),
                    palette::BLURPLE,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;
            }
            None => {
                let mut message = generate_embed(
                    String::from("Bruh"),
                    String::from("Nothing is playing."),
                    palette::BLURPLE,
                    Some(msg),
                    None,
                );

                msg.channel_id.send_message(ctx, |_| &mut message).await;
            }
        }
    }

    Ok(())
}

#[command]
#[aliases("nextup", "upnext")]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    let queue = songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call")
        .lock()
        .await
        .queue()
        .clone();

    let mut msg_content = String::new();

    // add each track to a new line in the message
    if queue.current_queue().len() > 0 {
        for (i, track) in queue.current_queue().iter().enumerate() {
            let track_str = mediautils::construct_title_string(track.metadata());

            if track.uuid() == queue.current().unwrap().uuid() {
                msg_content.push_str(&format!("**{}. {} - Now Bumpin'**\n", i + 1, track_str));
            } else {
                msg_content.push_str(&format!("{}. {}\n", i + 1, track_str));
            }
        }
    } else {
        msg_content.push_str("Empty stick. :(");
    }

    let mut message = generate_embed(
        String::from("Stick"),
        format!("\n{}", msg_content),
        palette::BLURPLE,
        Some(msg),
        None,
    );

    msg.channel_id.send_message(ctx, |_| &mut message).await;

    Ok(())
}

#[command]
#[aliases("loop")]
#[only_in(guilds)]
async fn repeat(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.expect("No guild").id;

    // we should guarantee that hes, like, in there, yknow fam

    let current_track = songbird::get(ctx)
        .await
        .expect("No songbird")
        .get(guild_id)
        .expect("Could not get songbird call")
        .lock()
        .await
        .queue()
        .current();

    match current_track {
        Some(track) => {
            if track.get_info().await.unwrap().loops == LoopState::Infinite {
                track.disable_loop();
            } else {
                track.enable_loop();
            }
            msgutils::success_react(ctx, msg).await;
        }
        None => {
            let mut message = generate_embed(
                String::from("You guys want some bagels?"),
                String::from("Buck a piece. I'll sell you the whole stick. $5"),
                palette::RED,
                Some(msg),
                None,
            );

            msg.channel_id.send_message(ctx, |_| &mut message).await;
        }
    }

    Ok(())
}
