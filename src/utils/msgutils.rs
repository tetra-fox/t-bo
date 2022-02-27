use serenity::{
    builder::CreateMessage, client::Context, model::channel::Message, Result as SerenityResult,
};

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
