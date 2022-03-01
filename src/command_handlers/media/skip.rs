use std::time::Duration;

use crate::{
    palette,
    utils::{msgutils::*, *},
};

use serenity::framework::standard::Delimiter;
use songbird::tracks::LoopState;
use url::Url;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::input::restartable::Restartable;