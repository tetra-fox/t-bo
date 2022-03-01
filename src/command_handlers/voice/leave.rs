use serenity::{client::Context, model::id::GuildId};

pub async fn leave(ctx: &Context, guild_id: GuildId) {
    let songbird = songbird::get(ctx)
        .await
        .expect("No songbird (Is the library missing?)");

    songbird
        .remove(guild_id)
        .await
        .expect("Could not leave");
}
