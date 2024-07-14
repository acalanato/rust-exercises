use mongodb::{action::ListDatabases, options::{ClientOptions, ConnectionString, ResolverConfig}, Client};
use std::error::Error;
use std::env;
use tokio;

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }
    Ok(())
}
*/
async fn list<'a>() -> ListDatabases<'a> {
    let client_uri = env::var("MONGODB_URI").expect("env var MONGODB_URI not set!");
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("".to_string());
    let client = Client::with_options(client_options);
//    client.list_databases(filter, options)
}

fn main() {
    println!("Hello, world!");
}
