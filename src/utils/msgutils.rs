use serenity::{
    client::Context, model::{channel::Message, id::ChannelId}, Result as SerenityResult
};

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn send_handler(
    ctx: &Context,
    chan_id: ChannelId,
    mut content: serenity::builder::CreateMessage<'_>) {

    match chan_id.send_message(ctx, |_| &mut content).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    };
}

pub fn generate_embed<'a>(
    title: String,
    description: String,
    color: i32,
    reference_message: Option<&Message>,
    image: Option<&str>,
) -> serenity::builder::CreateMessage<'a> {
    let mut message = serenity::builder::CreateMessage::default();
    let mut embed = serenity::builder::CreateEmbed::default();

    embed.title(title).description(description).color(color);
    
    if let Some(image) = image {
        embed.image(image);
    }

    message.set_embed(embed);

    if let Some(ref_msg) = reference_message {
        message.reference_message(ref_msg);
    }

    message
}

pub async fn success_react(ctx: &Context, msg: &Message) {
    let _ = msg.react(&ctx.http, '👌').await;
}
