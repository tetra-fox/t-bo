use std::{sync::Arc, time::Duration};

use serenity::{client::Context, prelude::Mutex, model::id::GuildId};
use songbird::{
    tracks::{TrackHandle, TrackState},
    Call,
};

pub async fn get_songbird_manager(ctx: &Context, guild_id: GuildId) -> Option<Arc<Mutex<Call>>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone()
        .get(guild_id);
    manager
}

pub async fn construct_np_msg(track: &TrackHandle) -> String {
    let default_unknown = &String::from("Unknown");
    let track_info = track.metadata();
    let track_title = track_info.title.as_ref().unwrap_or(default_unknown);
    let track_artist = track_info.artist.as_ref().unwrap_or(default_unknown);
    let track_url = track_info.source_url.as_ref().unwrap();
    // theoretically track_url should never error because the only way for the bot to play something is with from a url.

    let track_length = track_info.duration.unwrap_or(Duration::new(0, 0));
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

    let mut progress_bar = String::from("===================="); // 20 long
    let progress_percentage =
        ((track_position.as_secs() as f64 / track_length.as_secs() as f64) * 20.0) as usize;

    progress_bar =
        "#".repeat(progress_percentage) + &progress_bar[progress_percentage..progress_bar.len()];

    return format!(
        "[**{} - {}**]({})\n{} / {} **`[{}]`**",
        track_artist, track_title, track_url, track_pos_str, track_length_str, progress_bar
    );
}
