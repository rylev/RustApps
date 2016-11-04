extern crate libc;
pub mod capi;


trait TwitterClient {
    fn get(&mut self) -> Vec<Tweet>;
}

#[repr(C)]
pub struct Twitter {}

impl Twitter {
    fn new() -> Twitter {
        Twitter {}
    }
}

impl TwitterClient for Twitter {
    fn get(&mut self) -> Vec<Tweet> {
        vec![Tweet::new("Ryan Levick".to_owned(), "Some Text".to_owned())]
    }
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


#[test]
fn twitter_returns_nonempty_vector() {
    let mut twitter = Twitter::new();
    assert!(!twitter.get().is_empty());
}
