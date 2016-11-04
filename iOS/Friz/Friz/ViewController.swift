//
//  ViewController.swift
//  Friz
//
//  Created by Ryan Levick on 03/09/16.
//  Copyright © 2016 RyanLevick. All rights reserved.
//

import UIKit

class ViewController: UIViewController, UITableViewDataSource, UITableViewDelegate {
    
    var tweets: Array<Tweet> = []
    
    func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return tweets.count
    }
    
    func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
        let cell =  UITableViewCell()
        let tweet = tweets[indexPath.item]
        let name = tweet.name
        let text = tweet.text
        cell.textLabel?.text = "\(name) tweeted \(text)"
        return cell
    }
    
    func tableView(tableView: UITableView, didSelectRowAtIndexPath indexPath: NSIndexPath) {
        print("selecting things")
    }

    override func viewDidLoad() {
        let client = TwitterClient()
        self.tweets = client.fetchTweets()
        super.viewDidLoad()
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }

}

class TwitterClient {
    let clientPointer = twitter_create()
  
    func fetchTweets() -> Array<Tweet> {
        var tweets: Array<Tweet> = []
        let iterator = tweet_iter_create(clientPointer)
        var tweetPointer = tweet_iter_next(iterator)
        while tweetPointer != nil {
            let tweet = tweetFromPointer(tweetPointer)
            tweets.append(tweet)
            tweet_destroy(tweetPointer)
            tweetPointer = tweet_iter_next(iterator)
        }
        tweet_iter_destroy(iterator)
        return tweets
    }
    
    func tweetFromPointer(pointer: UnsafeMutablePointer<Void>) -> Tweet {
        let name = tweet_get_username(pointer)
        let text = tweet_get_text(pointer)
        return Tweet(name: name.asString()!, text: text.asString()!)
    }
    
    deinit {
        twitter_destroy(clientPointer)
    }
}

struct Tweet {
    let name: String
    let text: String
}
