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
    var list : FFIArray<Tweet>! = nil
    
    func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return list.count()
    }
    
    func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
        let cell =  UITableViewCell()
        let index = indexPath.item
        let tweet = self.list[index]
        let name = tweet!.username()
        let text = tweet!.text()
        cell.textLabel?.text = "\(name) tweeted \(text)"
        return cell
    }
    
    func tableView(tableView: UITableView, didSelectRowAtIndexPath indexPath: NSIndexPath) {
        print("selecting things")
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        let wrap = { (pointer) -> Tweet? in Tweet(pointer: pointer) }
        let create = { () in tweet_list_create(self.client) }
        self.list = FFIArray<Tweet>(create: create, access: tweet_list_get, size: tweet_list_len, wrap: wrap, destroy: tweet_list_destroy)
    }
    
    deinit {
        twitter_destroy(self.client)
    }
    
    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }
}

struct Tweet {
    let pointer: UnsafeMutablePointer<Void>
    
    func username() -> String {
        return tweet_get_username(self.pointer).asString()!
    }
    
    func text() -> String {
        return tweet_get_text(self.pointer).asString()!
    }
}

class FFIArray<T> {
    let pointer: UnsafeMutablePointer<Void>
    let access: (UnsafeMutablePointer<Void>, size_t) -> UnsafeMutablePointer<Void>
    let wrap: (UnsafeMutablePointer<Void>) -> T?
    let size: (UnsafeMutablePointer<Void>) -> size_t
    let destroy: (UnsafeMutablePointer<Void>) -> Void
    
    init (
        create: () -> UnsafeMutablePointer<Void>,
        access: (UnsafeMutablePointer<Void>, size_t) -> UnsafeMutablePointer<Void>,
        size: (UnsafeMutablePointer<Void>) -> size_t,
        wrap: (UnsafeMutablePointer<Void>) -> T?,
        destroy: (UnsafeMutablePointer<Void>) -> Void
    ) {
        self.wrap = wrap
        self.destroy = destroy
        self.size = size
        self.access = access
        pointer = create()
    }
    
    func count() -> Int {
        return self.size(self.pointer) as Int
    }
    
    subscript(index: Int) -> T? {
        let index = index as size_t
        let item = self.access(self.pointer, index)
        if item != nil {
            return self.wrap(item)
        } else {
            return nil
        }
    }
    
    deinit {
        if self.pointer != nil {
            self.destroy(self.pointer)
        }
    }

}
