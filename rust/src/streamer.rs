use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use ::wakeup::{WakeupBuilder, WakeupReceiver};
use ::{TwitterClient, Tweet};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TwitterStreamer {
  wakeup_receiver: Box<WakeupReceiver>,
  receiver: Receiver<Vec<Tweet>>,
  tweet_buffer: Vec<Tweet>,
  event_handler: Option<Box<Fn(u32)>>,
  new_tweet_count: u32
}

impl TwitterStreamer {

  pub fn start<F, W>(mut fetcher: F, wakeup: &mut W) -> TwitterStreamer where F: TwitterClient + 'static, W: WakeupBuilder {
    
    let (wakeup_tx, wakeup_rx) = wakeup.create_wakeup_channel();
    
    let (tx, rx) = channel();

    thread::spawn(move || {
      let mut most_recent_id : Option<u64> = None;
      loop {
        let tweets = fetcher.get(most_recent_id);
        
        let first_id = tweets.first().map(|ref t| t.id);
        if first_id.is_some() {
          most_recent_id = first_id;
        }

        tx.send(tweets).unwrap();
        wakeup_tx.wakeup();
        //twitter api is rate limited to 15 requests per 15 minutes
        thread::sleep(Duration::new(65, 0));
      }
    });

    let mut streamer = RcRefCell::new(TwitterStreamer {
      wakeup_receiver: wakeup_rx,
      receiver: rx,
      tweet_buffer: Vec::new(),
      event_handler: None,
      new_tweet_count: 0
    });

    let 
    streamer.wakeup_receiver.set_wakeup_handler(Some(Box::new(|| {
      streamer.on_new_tweets();
    })));

    streamer
  }

  pub fn set_new_tweets_handler(&mut self, handler: Option<Box<Fn(u32)>>) {
    self.event_handler = handler;
  }

  pub fn clone_list(&mut self) -> Vec<Tweet> {
    self.new_tweet_count = 0;
    self.tweet_buffer.clone()
  }

  fn register_handler(&mut self) {
    self.
  }

  fn on_new_tweets(&mut self) {
    while let Ok(tweets) = self.receiver.try_recv() {
      self.new_tweet_count += tweets.len() as u32;

      for tweet in tweets.into_iter() {
        self.tweet_buffer.insert(0, tweet);
      }
    }
    if let Some(ref handler) = self.event_handler {
      handler(self.new_tweet_count);
    }
  }
}
