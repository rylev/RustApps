#![feature(proc_macro)]

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;


pub mod capi;
pub mod twitter;
pub mod wakeup;

pub use capi::*;
pub use twitter::TwitterAPIClient;

trait TwitterClient {
    fn get(&mut self, since_id: Option<u64>) -> Vec<Tweet>;
}

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
