use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use ::wakeup::WakeupBuilder;
use ::TwitterClient;

pub struct TwitterStreamer {
  wakeup_receiver: WakeupReceiver,
  receiver: Receiver<Vec<Tweet>>,
  tweet_buffer: Vec<Tweet>,
  event_handler: Option<foo>,
  new_tweet_count: u32
}

impl TwitterStreamer {

  pub fn new(fetcher: F, wakeup: &mut W) -> TwitterStreamer where F: TwitterClient, W: WakeupBuilder {
    
    let (wakeup_tx, wakeup_rx) = wakeup.create_wakeup_channel(|| {
      self.on_new_tweets();
    });
    
    let (tx, rx) = channel();

    thread::spawn(move || {
      let most_recent_id : Option<u64> = None;
      loop {
        let tweets = fetcher.get(most_recent_id);
        
        let first_id = tweets.first().map(|ref t| t.id);
        if let Some(id) = first_id {
          most_recent_id = Some(id);
        }

        tx.send(tweets).unwrap();
        wakeup_tx.wakeup();
        //twitter api is rate limited to 15 requests per 15 minutes
        thread::sleep(Duration::new(65, 0));
      }
    });

    TwitterStreamer {
      wakeup_receiver: wakeup_rx,
      receiver: rx,
      tweet_buffer: Vec::new(),
      event_handler: None,
      new_tweet_count: 0
    }
  }

  pub fn set_new_tweets_handler(handler) {
    self.event_handler = handler;
  }

  pub fn clone_list(&mut self) -> Vec<Tweet> {
    self.new_tweet_count = 0;
    self.tweet_buffer.clone()
  }

  fn on_new_tweets(&mut self) {
    while let Ok(tweets) = self.receiver.try_recv() {
      for tweet in tweets.into_iter() {
        self.tweet_buffer.insert(0, tweet);
      }
      self.new_tweet_count += tweets.len();
    }
    (self.event_handler)(self.new_tweet_count);
  }
}
