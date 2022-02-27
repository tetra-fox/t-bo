use std::time::Duration;

use serenity::{
    builder::CreateMessage, client::Context, model::channel::Message, Result as SerenityResult,
};
use songbird::tracks::{TrackHandle, TrackState};

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub fn create_embed_message(
    m: &mut CreateMessage,
    title: &String,
    description: &String,
    color: i32,
    reference_message: Option<&Message>,
) {
    if let Some(ref_msg) = reference_message {
        m.reference_message(ref_msg);
    }

    m.embed(|e| e.title(title).description(description).color(color));
}

pub async fn success_react(ctx: &Context, msg: &Message) {
    let _ = msg.react(&ctx.http, '👌').await;
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

    progress_bar = "#".repeat(progress_percentage)
        + &progress_bar[progress_percentage..progress_bar.len()];

    return format!(
        "[**{} - {}**]({})\n{} / {} **`[{}]`**",
        track_artist, track_title, track_url, track_pos_str, track_length_str, progress_bar
    );
}
