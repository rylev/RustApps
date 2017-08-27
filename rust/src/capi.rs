use super::{AppRoot, TwitterAPIClient, TwitterClient, Tweet};
use super::streamer::TwitterStreamer;
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
/*
pub struct CEventCallback<T> {
    event_ctx: *mut c_void,
    callback: extern "C" fn(ctx: *mut c_void, event: T)
}

impl<T> Fn<(T)> for CEventCallback<T> {
    fn call(&self, args: T) {
        (self.callback)(self.event_ctx, event.0);
    }
}
*/

#[no_mangle]
pub extern "C" fn twitter_create() -> *mut CTwitterClient {
    let mut wakeup_builder = ::wakeup::create_wakeup_builder();
    let client = TwitterAPIClient {};
    let streamer = TwitterStreamer::start(client, &mut wakeup_builder);
    let root = AppRoot {
        wakeup_builder: wakeup_builder,
        twitter_streamer: streamer
    };
    let root_box = Box::new(root);

    Box::into_raw(root_box) as *mut CTwitterClient
}

#[no_mangle]
pub unsafe extern "C" fn twitter_set_event_handler(
    twitter: *mut CTwitterClient,
    event_ctx: *mut c_void,
    callback: Option<extern "C" fn(*mut c_void, CTweetListEvent)>
) {
    let mut root = Box::from_raw(twitter as *mut AppRoot);
    
    if let Some(callback_fn) = callback {
        let closure = Box::new(move |tweet_count: u32| {
            callback_fn(event_ctx, CTweetListEvent { count: tweet_count });
        });
        root.twitter_streamer.set_new_tweets_handler(Some(closure));
    }
    else {
        root.twitter_streamer.set_new_tweets_handler(None);
    }

    Box::into_raw(root);
}



#[no_mangle]
pub unsafe extern "C" fn twitter_destroy(root: *mut CTwitterClient) {
    Box::from_raw(root as *mut AppRoot);
}

#[no_mangle]
pub unsafe extern "C" fn tweet_list_create(root: *mut CTwitterClient) -> *mut CTweetList {
    let mut root = Box::from_raw(root as *mut AppRoot);
    let vec = root.twitter_streamer.clone_list();
    //TODO: root gets dropped here?
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