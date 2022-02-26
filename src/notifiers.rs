use crate::msgutils::*;

use serenity::{async_trait, http::Http, model::prelude::ChannelId};

use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use std::sync::Arc;

pub struct TrackEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            check_msg(
                self.chan_id
                    .say(&self.http, &format!("Tracks ended: {}.", track_list.len()))
                    .await,
            );
        }

        None
    }
}
