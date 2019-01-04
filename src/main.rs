#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate kankyo;

use std::env;

use serenity::{
    framework::StandardFramework,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    kankyo::load().expect("Failed to load ./.env file");
    env_logger::init().expect("Failed to initialize env_logger");
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .cmd("ping", ping));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}

command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong!");
});
