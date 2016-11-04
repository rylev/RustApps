use super::{TwitterAPIClient, TwitterClient, Tweet};
use std;
use std::os::raw::{c_char};
use std::ffi::CStr;

#[repr(C)]
pub struct RustByteSlice {
    bytes: *const u8,
    length: usize,
}

#[no_mangle]
pub extern "C" fn rust_print(c_string_pointer: *const c_char) {
    let cstring = unsafe { CStr::from_ptr(c_string_pointer) };
    print_bytes(cstring.to_bytes())
}

fn print_bytes(bytes: &[u8]) {
    if let Ok(string) = std::str::from_utf8(bytes) {
        println!("{}", string)
    }
}

#[no_mangle]
pub extern "C" fn twitter_create() -> *mut TwitterAPIClient {
    let twitter = Box::new(TwitterAPIClient {});

    Box::into_raw(twitter)
}

#[no_mangle]
pub unsafe extern "C" fn twitter_destroy(twitter: *mut TwitterAPIClient) {
    Box::from_raw(twitter);
}

pub type TweetList = std::vec::Vec<Tweet>;
#[no_mangle]
pub unsafe extern "C" fn tweet_list_create(twitter: *mut TwitterAPIClient) -> *mut TweetList {
    let mut twitter = Box::from_raw(twitter);
    let vec = twitter.get();
    Box::into_raw(Box::new(vec))
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_destroy(tweet_iter: *mut TweetList) {
    Box::from_raw(tweet_iter);
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_get(tweet_list: *mut TweetList, index: usize) -> *const Tweet {
    let tweet_list = tweet_list as *mut TweetList;
    let vec = Box::from_raw(tweet_list);

    let ptr : *const Tweet = vec.get(index)
        .map(|t| t as *const Tweet)
        .unwrap_or(std::ptr::null());
    // Covert back to raw pointer so vec won't be dropped
    Box::into_raw(vec);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_len(tweet_list: *mut TweetList) -> usize {
    let tweet_list = tweet_list as *mut TweetList;
    let list = Box::from_raw(tweet_list);
    let len = list.len();
    // Covert back to raw pointer so vec won't be dropped
    Box::into_raw(list);
    len
}

#[no_mangle]
pub unsafe extern "C" fn tweet_get_username(tweet: *mut Tweet) -> RustByteSlice {
    let tweet = Box::from_raw(tweet);
    let slice = {
        let name = &tweet.username;
        RustByteSlice {
            bytes: name.as_ptr(),
            length: name.len()
        }
    };
    Box::into_raw(tweet);
    slice
}

#[no_mangle]
pub unsafe extern "C" fn tweet_get_text(tweet: *mut Tweet) -> RustByteSlice {
    let tweet = Box::from_raw(tweet);
    let slice = {
        let text = &tweet.text;
        RustByteSlice {
            bytes: text.as_ptr(),
            length: text.len()
        }
    };
    Box::into_raw(tweet);
    slice
}

#[cfg(test)]
mod tests {

    use super::*;
    use std;

    #[test]
    fn get_username() {
        unsafe {
            let twitter_ptr = twitter_create();
            let iter_ptr = tweet_iter_create(twitter_ptr);
            let tweet_ptr = tweet_iter_next(iter_ptr);

            assert!(!tweet_ptr.is_null());

            let username_buffer = tweet_get_username(tweet_ptr);
            let username_slice = std::slice::from_raw_parts(username_buffer.bytes, username_buffer.length);
            let username = std::str::from_utf8(username_slice).unwrap();
            assert!(username == "Ryan Levick");

            let null_ptr = tweet_iter_next(iter_ptr);
            assert!(null_ptr.is_null());

            tweet_destroy(tweet_ptr);
            tweet_iter_destroy(iter_ptr);
            twitter_destroy(twitter_ptr);
        }
    }
}