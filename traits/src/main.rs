//A trait defines functionality a particular type has and can
//share with other types. We can use traits to define shared
//behavior in an abstract way. We can use trait bounds to
//specify that a generic type can be any type that has certain behavior.

use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_shoes"),
        content:  String::from(
            "i won't do what you tell me"
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new xweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Capivaras voadoras"),
        location: String::from("Brumenau"),
        author:   String::from("John Doe"),
        content:  String::from(
            "Capivaras atacaram um evento gospel, seria um sinal do fim dos tempos?"
        ),
    };
    println!("New article available! {}", article.summarize());
}
