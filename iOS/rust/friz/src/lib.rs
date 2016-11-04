#![feature(proc_macro)]

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;


pub mod capi;
pub mod twitter;

pub use capi::*;
pub use twitter::TwitterAPIClient;

trait TwitterClient {
    fn get(&mut self) -> Vec<Tweet>;
}

pub struct Tweet {
    username: String,
    text: String
}

impl Drop for Tweet {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

impl Tweet {
    fn new(username: String, text: String) -> Tweet {
        Tweet { username: username, text: text }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
