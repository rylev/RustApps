use super::{TwitterAPIClient, TwitterClient, Tweet};
use std;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::vec::Vec;

#[repr(C)]
pub struct RustByteSlice {
    bytes: *const u8,
    length: usize,
}
pub type CTwitterClient = ();
pub type CTweetList = ();
pub type CTweet = ();

#[repr(C)]
pub struct CTweetListEvent {
    count: u32
}

#[no_mangle]
pub extern "C" fn twitter_create() -> *mut CTwitterClient {
    let twitter = Box::new(TwitterAPIClient {});

    Box::into_raw(twitter) as *mut CTwitterClient
}

#[no_mangle]
pub unsafe extern "C" fn twitter_set_event_handler(
    twitter: *mut CTwitterClient,
    event_ctx: *mut c_void,
    callback: extern "C" fn(*mut c_void, CTweetListEvent)
) {
    let mut twitter = Box::from_raw(twitter as *mut TwitterAPIClient);
    
    Box::into_raw(twitter);
}



#[no_mangle]
pub unsafe extern "C" fn twitter_destroy(twitter: *mut CTwitterClient) {
    Box::from_raw(twitter as *mut TwitterAPIClient);
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_create(twitter: *mut CTwitterClient) -> *mut CTweetList {
    let mut twitter = Box::from_raw(twitter as *mut TwitterAPIClient);
    let vec = twitter.get(None);
    Box::into_raw(Box::new(vec)) as *mut CTweetList
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_destroy(tweet_iter: *mut CTweetList) {
    Box::from_raw(tweet_iter as *mut Vec<Tweet>);
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_get(tweet_list: *mut CTweetList, index: usize) -> *const CTweet {
    let tweet_list = tweet_list as *mut Vec<Tweet>;
    let vec = Box::from_raw(tweet_list);

    let ptr : *const Tweet = vec.get(index)
        .map(|t| t as *const Tweet)
        .unwrap_or(std::ptr::null());
    // Covert back to raw pointer so vec won't be dropped
    Box::into_raw(vec);
    ptr as *const CTweet
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_len(tweet_list: *mut CTweetList) -> usize {
    let tweet_list = tweet_list as *mut Vec<Tweet>;
    let list = Box::from_raw(tweet_list);
    let len = list.len();
    // Covert back to raw pointer so vec won't be dropped
    Box::into_raw(list);
    len
}

#[no_mangle]
pub unsafe extern "C" fn tweet_get_username(tweet: *mut CTweet) -> RustByteSlice {
    let tweet = Box::from_raw(tweet as *mut Tweet);
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
pub unsafe extern "C" fn tweet_get_text(tweet: *mut CTweet) -> RustByteSlice {
    let tweet = Box::from_raw(tweet as *mut Tweet);
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