//! src/main.rs
use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind to random port.");
    println!("Bound on port: {}", configuration.application_port);
    startup::run(listener, db_pool)?.await
}
