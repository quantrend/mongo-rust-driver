#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate mongodb;

#[cfg(feature = "tokio-runtime")]
// CONNECTION EXAMPLE STARTS HERE
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    use mongodb::{options::ClientOptions, Client};
    let client_options = ClientOptions::parse(
        "mongodb+srv://quantrend-dev-aws.vcrsn.mongodb.net/quantrend-dev-aws?\
        authMechanism=MONGODB-AWS&authSource=$external",
    )
    .await?;
    let client = Client::with_options(client_options)?;
    println!("{:?}", client.list_database_names(None, None).await?);
    Ok(())
}
// CONNECTION EXAMPLE ENDS HERE
