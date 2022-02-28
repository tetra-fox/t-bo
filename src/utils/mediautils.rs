use std::{sync::Arc, time::Duration};

use serenity::{client::Context, model::id::GuildId, prelude::Mutex};
use songbird::{
    tracks::{TrackHandle, TrackState},
    Call, input::Metadata,
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
    let track_artist = metadata.channel.as_deref().unwrap_or(default_unknown);
    let track_url = metadata.source_url.as_deref().unwrap();
    // theoretically track_url should never error because the only way for the bot to play something is with from a url.

    if track_title.starts_with(&format!("{} - ", track_artist)) {
        track_title = &track_title[track_artist.len() + 3 as usize..];
    }

    return format!("[{} - {}]({})", track_artist, track_title, track_url);
}
