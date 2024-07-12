use std::io;
use tokio;
use mongodb::{self, action::{ListDatabases, ListNames}, bson::Document, Client};


async fn list<'a>() -> ListDatabases<'a> {
    let client = Client::with_uri_str("").await
        .ok()
        .expect("No connection to the server");

    let client_item = client.clone();
    client_item.list_databases()
}

fn main() {
    println!("Hello, world!");
}
