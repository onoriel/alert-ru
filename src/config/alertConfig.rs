use dotenv::dotenv;
use std::env;

pub struct AlertConfig {
    pub mongo_url: String,
    pub db_name: String,
}

impl AlertConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set in .env file");
        let db_name = env::var("DB_NAME").expect("DB_NAME not set in .env file");

        AlertConfig { mongo_url, db_name }
    }
}