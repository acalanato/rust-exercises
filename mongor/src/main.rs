use mongodb::{Client,options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use std::env;
use tokio;


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

async fn list<'a>() -> ListDatabases<'a> {
    let client = Client::with_uri_str("").await
        .ok()
        .expect("No connection to the server");

    let client_item = client.clone();
    client_item.list_databases()
}

//fn main() {
//    println!("Hello, world!");
//}
