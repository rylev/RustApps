#![feature(proc_macro)]

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;


pub mod capi;
pub mod twitter;
pub mod wakeup;
pub mod streamer;

pub use capi::*;
pub use twitter::TwitterAPIClient;


pub struct AppRootTemplate<W: wakeup::WakeupBuilder> {
    wakeup_builder: W,
    twitter_streamer: std::cell::RefCell<streamer::TwitterStreamer>
}

pub type AppRoot = AppRootTemplate<wakeup::WakeupBuilderImpl>;

pub trait TwitterClient : Send {
    fn get(&mut self, since_id: Option<u64>) -> Vec<Tweet>;
}

#[derive(Clone)]
pub struct Tweet {
    username: String,
    text: String,
    id: u64
}

impl Drop for Tweet {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

impl Tweet {
    fn new(username: String, text: String, id: u64) -> Tweet {
        Tweet { username: username, text: text, id: id }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
