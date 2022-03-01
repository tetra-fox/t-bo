use std::{time::Duration, sync::Arc};

use crate::{
    palette,
    utils::{msgutils::*, *},
};

use serenity::{framework::standard::Delimiter, model::id::GuildId, prelude::Mutex};
use songbird::{tracks::LoopState, Call};
use url::Url;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::input::restartable::Restartable;

pub async fn play_url(call: &Arc<Mutex<Call>>, url: String, queue_position: usize) {
    let mut call_lock = call.lock().await;

    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => source,
        Err(e) => {
            println!("This shit broke: {:?}", e);

            return;
        }
    };

    if queue_position != 0 {
        // move to position
        // call_lock.queue().current_queue().insert(queue_position, source.into());
    } else {
        // just add to end
        call_lock.enqueue_source(source.into());
    }
}

pub async fn play_search(call: &Arc<Mutex<Call>>, query: String, queue_position: usize) {
    let mut call_lock = call.lock().await;

    let source = match Restartable::ytdl_search(query, true).await {
        Ok(source) => source,
        Err(e) => {
            println!("This shit broke: {:?}", e);

            return;
        }
    };

    if queue_position != 0 {
        // move to position
        // TBI: probably add to end, then move
        // call_lock.queue().current_queue().insert(queue_position, source.into());
    } else {
        // just add to end
        call_lock.enqueue_source(source.into());
    }
}
