//
//  ViewController.swift
//  Friz
//
//  Created by Ryan Levick on 03/09/16.
//  Copyright © 2016 RyanLevick. All rights reserved.
//

import UIKit

class ViewController: UIViewController, UITableViewDataSource, UITableViewDelegate {
    
    @IBOutlet weak var updateButton: UIButton!
    var client : UnsafeMutablePointer<CTwitterClient> = nil
    var list : FFIArray<CTweetList, CTweet, Tweet>! = nil
    
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
    
    func onListUpdated(evt: UnsafePointer<CTwitterEvent>) {
        updateButton.titleLabel!.text = "\(evt.memory.count) new tweets";
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        client = twitter_create();
        twitter_set_event_handler(client, to_ptr(self), { (ctx, evt) in
            let o : ViewController = from_ptr(ctx);
            o.onListUpdated(evt);
        });

        let list_handle = tweet_list_create(self.client);
        self.list = FFIArray(
            handle: list_handle,
            access: tweet_list_get,
            size: tweet_list_len,
            wrap: { Tweet(pointer: $0) },
            destroy: tweet_list_destroy
        )
    }
    
    deinit {
        twitter_destroy(self.client)
    }
    
    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }
}

func to_ptr<T: AnyObject>(o: T) -> UnsafeMutablePointer<Void> {
    return UnsafeMutablePointer(Unmanaged.passUnretained(o).toOpaque());
}

func from_ptr<T: AnyObject>(ptr: UnsafeMutablePointer<Void>) -> T {
    return Unmanaged<T>.fromOpaque(COpaquePointer(ptr)).takeUnretainedValue();
}

struct Tweet {
    let pointer: UnsafeMutablePointer<CTweet>
    
    func username() -> String {
        return tweet_get_username(self.pointer).asString()!
    }
    
    func text() -> String {
        return tweet_get_text(self.pointer).asString()!
    }
}

class FFIArray<CList, CItem, Item> {

    typealias ListHandle = UnsafeMutablePointer<CList>
    typealias ItemHandle = UnsafeMutablePointer<CItem>

    
    let handle: ListHandle
    let access: (ListHandle, size_t) -> ItemHandle
    let wrap: (ItemHandle) -> Item?
    let size: (ListHandle) -> size_t
    let destroy: (ListHandle) -> Void
    
    init (
        handle: ListHandle,
        access: (ListHandle, size_t) -> ItemHandle,
        size: (ListHandle) -> size_t,
        wrap: (ItemHandle) -> Item?,
        destroy: (ListHandle) -> Void
    ) {
        self.wrap = wrap
        self.destroy = destroy
        self.size = size
        self.access = access
        self.handle = handle
    }
    
    func count() -> Int {
        return self.size(self.handle) as Int
    }
    
    subscript(index: Int) -> Item? {
        let index = index as size_t
        let item = self.access(self.handle, index)
        if item != nil {
            return self.wrap(item)
        } else {
            return nil
        }
    }
    
    deinit {
        if self.handle != nil {
            self.destroy(self.handle)
        }
    }

}
