enum TBoResult {
    Ok(&'static str, &'static str),
    Warn(&'static str, &'static str),
    Err(&'static str, &'static str)
}

impl TBoResult {
    fn notify(&self) {
        let mut message = generate_embed(
            String::from("Why'd u send a jpg"),
            String::from("That's a jpg bro"),
            palette::RED,
            Some(msg),
        );
    
        msg.channel_id.send_message(ctx, |_| &mut message).await;
    }
}
