use super::{Twitter, TwitterClient, Tweet};
use std;
use std::os::raw::{c_char};
use std::ffi::CStr;
use libc::size_t;

#[repr(C)]
pub struct RustByteSlice {
    bytes: *const u8,
    length: size_t,
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
pub extern "C" fn twitter_create() -> *mut Twitter {
    let twitter = Box::new(Twitter::new());

    Box::into_raw(twitter)
}

#[no_mangle]
pub unsafe extern "C" fn twitter_destroy(twitter: *mut Twitter) {
    Box::from_raw(twitter);
}

pub type TweetIter = std::vec::IntoIter<Tweet>;
#[no_mangle]
pub unsafe extern "C" fn tweet_iter_create(twitter: *mut Twitter) -> *mut TweetIter {
    let mut twitter = Box::from_raw(twitter);
    let vec = twitter.get();
    Box::into_raw(Box::new(vec.into_iter()))
}

#[no_mangle]
pub unsafe extern "C" fn tweet_iter_destroy(tweet_iter: *mut TweetIter) {
    Box::from_raw(tweet_iter);
}

#[no_mangle]
pub unsafe extern "C" fn tweet_iter_next(twitter_result: *mut TweetIter) -> *mut Tweet {
    let twitter_result = twitter_result as *mut std::vec::IntoIter<Tweet>;
    let mut iter = Box::from_raw(twitter_result);
    let ptr = iter.next().
        map(|t| Box::into_raw(Box::new(t))).
        unwrap_or(std::ptr::null_mut());

    // Covert back to raw pointer so iter won't be dropped
    Box::into_raw(iter);
    ptr
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
pub unsafe extern "C" fn tweet_destroy(tweet: *mut Tweet) {
    Box::from_raw(tweet);
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