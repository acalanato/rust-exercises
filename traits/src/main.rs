//A trait defines functionality a particular type has and can
//share with other types. We can use traits to define shared
//behavior in an abstract way. We can use trait bounds to
//specify that a generic type can be any type that has certain behavior.

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_shoes"),
        content:String::from(
            "i won't do what you tell me"
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new xweet: {}", tweet.summarize());
}
