use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

pub async fn init_db() -> Result<Client, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match ClientOptions::parse(&database_url).await {
        Ok(mut client_options) => {
            client_options.app_name = Some("social_app".to_string());
            Client::with_options(client_options).map_err(|e| e.into())
        }
        Err(e) => {
            log::error!("Failed to parse MongoDB URL: {}", e);
            Err(e.into())
        }
    }
}

