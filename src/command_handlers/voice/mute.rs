use serenity::{client::Context, model::id::GuildId};
use serenity::prelude::Mutex;
use tracing::{event, Level};
use std::sync::Arc;
use songbird::Call;

pub async fn mute(call_m: &Arc<Mutex<Call>>) {
    let mut call = call_m.lock().await;

    if call.is_mute() {
        let _ = call.mute(false).await;
    } else {
        let _ = call.mute(true).await;
    }
}
