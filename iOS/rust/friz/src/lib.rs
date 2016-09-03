extern crate egg_mode;

use std::str;
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern fn rust_print(c_string_pointer: *const c_char) {
    let cstring = unsafe { CStr::from_ptr(c_string_pointer) };
    print_bytes(cstring.to_bytes())
}

fn print_bytes(bytes: &[u8]) {
    if let Ok(string) = str::from_utf8(bytes) {
        println!("{}", string)
    }
}

pub struct Tweet {
    username: String,
    text: String
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

pub struct Twitter {
    consumer_token: egg_mode::Token<'static>
}

impl Twitter {
    pub fn new () -> Twitter {
        let consumer_key = "";
        let consumer_secret = "";
        let consumer_token = egg_mode::Token::new(consumer_key, consumer_secret);
        Twitter { consumer_token: consumer_token }
    }

    pub fn tweets (&self, access_token: egg_mode::Token<'static>) -> Vec<Tweet> {
        let mut timeline = egg_mode::tweet::home_timeline(&self.consumer_token, &access_token).with_page_size(10);

        let mut tweets = Vec::new();
        for tweet in &timeline.start().unwrap().response {
            tweets.push(Tweet::new(tweet.user.screen_name.clone(), tweet.text.clone()));
        }

        tweets
    }
}
