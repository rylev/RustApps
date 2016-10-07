extern crate egg_mode;
extern crate libc;

use std::str;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use libc::size_t;

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

#[no_mangle]
pub extern fn twitter_create() -> *mut Twitter {
    let twitter = Box::new(Twitter::new());

    Box::into_raw(twitter)
}

#[no_mangle]
pub unsafe extern fn twitter_destroy(twitter: *mut Twitter) {
    Box::from_raw(twitter);
}

pub type TweetIter = ();
#[no_mangle]
pub unsafe extern fn twitter_get(twitter: *mut Twitter) -> *mut TweetIter {
    let mut twitter = Box::from_raw(twitter);
    let vec = twitter.get();
    let ptr = Box::into_raw(Box::new(vec.iter())) as *mut TweetIter;
    std::mem::forget(vec);
    ptr
}

pub type FFITweet = ();
#[no_mangle]
pub unsafe extern fn tweet_iter_next<'a>(twitter_result: *mut TweetIter) -> *mut FFITweet {
    let twitter_result = twitter_result as *mut std::slice::Iter<'a, Tweet>;
    let mut iter = Box::from_raw(twitter_result);
    let ptr = iter.next().
        map(|t| std::mem::transmute::<_, *mut FFITweet>(t)).
        unwrap_or(std::ptr::null_mut());

    // Covert back to raw pointer so iter won't be dropped
    Box::into_raw(iter);
    ptr
}

#[no_mangle]
pub unsafe extern fn tweet_get_username<'a>(tweet: *mut FFITweet) -> RustByteSlice {
    let tweet = std::mem::transmute::<_, &Tweet>(tweet);
    let name = &tweet.username;
    RustByteSlice {
        bytes: name.as_ptr(),
        length: name.len()
    }
}

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

#[repr(C)]
pub struct RustByteSlice {
    bytes: *const u8,
    length: size_t,
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

#[test]
fn capi_get_username() {
    unsafe {
        let twitter_ptr = twitter_create();
        let iter_ptr = twitter_get(twitter_ptr);
        let tweet_ptr = tweet_iter_next(iter_ptr);

        assert!(!tweet_ptr.is_null());

        let username_buffer = tweet_get_username(tweet_ptr);
        let username_slice = std::slice::from_raw_parts(username_buffer.bytes, username_buffer.length);
        let username = str::from_utf8_unchecked(username_slice);
        assert!(username == "Ryan Levick");

        let tweet_ptr = tweet_iter_next(iter_ptr);
        assert!(tweet_ptr.is_null());
    }
}
