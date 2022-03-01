use std::{any::Any, sync::Arc, time::Duration};

use serenity::{
    client::Context,
    model::{
        channel::{ChannelType, Message},
        id::{ChannelId, GuildId, UserId},
    },
    prelude::Mutex,
};
use songbird::{
    input::Metadata,
    tracks::{TrackHandle, TrackState},
    Call,
};

pub async fn get_songbird(ctx: &Context, guild_id: GuildId) -> Option<Arc<Mutex<Call>>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone()
        .get(guild_id);
    manager
}

pub async fn construct_np_msg(track: &TrackHandle) -> String {
    let title_string = construct_title_string(track.metadata());

    let track_length = track.metadata().duration.unwrap_or(Duration::new(0, 0));
    let track_position = track
        .get_info()
        .await
        .unwrap_or(TrackState::default())
        .position;

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

    let mut progress_bar = String::from("=".repeat(20));
    let progress_percentage =
        ((track_position.as_secs() as f64 / track_length.as_secs() as f64) * 20.0) as usize;

    progress_bar =
        "#".repeat(progress_percentage) + &progress_bar[progress_percentage..progress_bar.len()];

    return format!(
        "**{}**\n{} / {} **`[{}]`**",
        title_string, track_pos_str, track_length_str, progress_bar
    );
}

pub fn construct_title_string(metadata: &Metadata) -> String {
    let default_unknown = "Unknown";
    let mut track_title = metadata.title.as_deref().unwrap_or(default_unknown);

    // get Channel (usually for YouTube), if None, get Artist (other platforms), if None, then "Unknown"
    let track_artist = metadata
        .channel
        .as_deref()
        .unwrap_or(metadata.artist.as_deref().unwrap_or(default_unknown));

    let track_url = metadata.source_url.as_deref().unwrap();
    // theoretically track_url should never error because the only way for the bot to play something is with from a url.

    if track_title.starts_with(&format!("{} - ", track_artist)) {
        track_title = &track_title[track_artist.len() + 3 as usize..];
    }

    return format!("[{} - {}]({})", track_artist, track_title, track_url);
}

pub async fn get_voice_channel_of_user(ctx: &Context, msg: &Message) -> Option<ChannelId> {
    match msg
        .guild(&ctx.cache)
        .await
        .expect("No guild")
        .voice_states
        .get(&msg.author.id)
        .unwrap()
        .channel_id
    {
        Some(channel) => return Some(channel),
        None => return None,
    }
}
