use std::fmt::Display;

pub trait Summary {

    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
/*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
*/
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
/*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
*/    
}

//Traits as Parameters
//pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}

//Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//more itens
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
//or
//pub fn notify<T: Summary>(item1: &T, item2: &T) {}

/*
doesn't work?
fn my_generic_fn<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0_i32
}
*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("FieryMullets"),
        content:  String::from(
            "Mullets are so hot right now"
        ),
        reply: false,
        retweet: false,
    }
}
