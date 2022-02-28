use serenity::{async_trait, prelude::Context, http::Http, model::{prelude::ChannelId, id::GuildId}};

use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use std::sync::Arc;

use crate::{palette, utils::*};

use msgutils::*;

pub struct TrackEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
    pub guild_id: GuildId,
    pub songbird_context: Context,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_) = ctx {

            let queue = songbird::get(&self.songbird_context)
            .await
            .expect("No songbird")
            .get(self.guild_id)
            .expect("Could not get songbird call")
            .lock()
            .await
            .queue()
            .current_queue();

            if queue.len() < 1 {
                return None;
            }

            let msg_content = mediautils::construct_np_msg(&queue[0]).await;

            let mut message = generate_embed(
                String::from("We bumpin'"),
                msg_content,
                palette::BLURPLE,
                None,
                None
            );

            // msgutils::send_handler(&self.http, self.chan_id, message).await;

            self.chan_id.send_message(&self.http, |_| &mut message).await;
        }

        None
    }
}
