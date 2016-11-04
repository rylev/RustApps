//
//  ViewController.swift
//  Friz
//
//  Created by Ryan Levick on 03/09/16.
//  Copyright © 2016 RyanLevick. All rights reserved.
//

import UIKit

class ViewController: UIViewController, UITableViewDataSource, UITableViewDelegate {
    
    let client = twitter_create()
    var list : UnsafeMutablePointer<Void> = nil
    
    func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return tweet_list_len(self.list) as Int
    }
    
    func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
        let cell =  UITableViewCell()
        let index = indexPath.item as size_t
        let tweet = tweet_list_get(self.list, index)
        let name = tweet_get_username(tweet).asString()!
        let text = tweet_get_text(tweet).asString()!
        cell.textLabel?.text = "\(name) tweeted \(text)"
        return cell
    }
    
    func tableView(tableView: UITableView, didSelectRowAtIndexPath indexPath: NSIndexPath) {
        print("selecting things")
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.list = tweet_list_create(self.client)
    }
    
    deinit {
        if self.list != nil {
            tweet_list_destroy(self.list)
        }
        twitter_destroy(self.client)
    }
    
    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }

}
