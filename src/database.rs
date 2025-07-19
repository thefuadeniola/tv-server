use mongodb::{Client, Database};
use dotenv::dotenv;
use std::env;


pub async fn database_connection() -> Result<Database, mongodb::error::Error> {
    dotenv().ok();
    let uri = env::var("DATABASE_URI").expect("Failed to load connection string");

    let client = Client::with_uri_str(uri).await?;
    let database = client.database("uctvdb");

    Ok(database)
}