use std::sync::Arc;

use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
};

use serenity::prelude::Mutex;
use songbird::{Call, Event, TrackEvent};

use crate::notifiers;

pub async fn join(ctx: &Context, guild_id: GuildId, voice_id: ChannelId, bind_id: ChannelId) -> Arc<Mutex<Call>> {
    let songbird = songbird::get(ctx)
        .await
        .expect("No songbird (is ths library missing?)");

    let (call_m, success) = songbird.join(guild_id, voice_id).await;

    let mut call = call_m.lock().await;

    // track end handler, notify the channel that the bot is bound to of the new song
    call.add_global_event(
        Event::Track(TrackEvent::End),
        notifiers::TrackEndNotifier {
            chan_id: bind_id,
            http: ctx.http.clone(),
            guild_id: guild_id,
            songbird_context: ctx.clone(),
        },
    );

    // deafen to save us bandwidth
    // super::deafen(&call_m).await;

    success.expect("Failed to join channel");

    call_m.clone()
}
