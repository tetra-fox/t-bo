use serenity::{client::Context, model::id::GuildId};
use serenity::prelude::Mutex;
use tracing::{event, Level};
use std::sync::Arc;

pub async fn deafen(call: &Arc<Mutex<songbird::Call>>) {
    let mut call_lock = call.lock().await;

    tracing::info!("bnraauh");
    
    if call_lock.is_deaf() {
        tracing::info!("bnruh");
        let _ = call_lock.deafen(false).await;
    } else {
        tracing::info!("bnruafafah");
        let _ = call_lock.deafen(true).await;
    }
}
