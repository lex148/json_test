use crate::models::dog::Dog;
use std::env;

mod errors;
mod migrations;
mod models;

use welds::connections::postgres::PostgresClient;
pub(crate) type DbClient = actix_web::web::Data<PostgresClient>;

#[actix_web::main]
async fn main() -> welds::errors::Result<()> {
    // default log level to info
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    pretty_env_logger::init();
    if let Err(err) = dotenvy::dotenv() {
        match err {
            dotenvy::Error::Io(_) => {}
            _ => log::warn!("DOTENV: {:?}", err),
        }
    }

    // Connect to the database and run the migrations
    let connection_string = env::var("DATABASE_URL").unwrap();
    let client = welds::connections::postgres::connect(&connection_string)
        .await
        .expect("Unable to connect to Database");
    migrations::up(&client).await.unwrap();

    // print all the Dogs from the database
    let all = Dog::all().run(&client).await?;
    for dog in &all {
        println!("Dog: {:?}", dog);
    }

    let mut new_dog = welds::state::DbState::new_uncreated(Dog {
        id: 0,
        name: "Spot".to_string(),
        test_json: serde_json::json!({ "a": all.len() }),
    });

    new_dog.save(&client).await?;

    Ok(())
}
