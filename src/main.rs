//! src/main.rs
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, email_client::EmailClient, startup};

mod telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender,
        configuration.email_client.authorization_token,
    )
    .await;

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind to random port.");
    tracing::info!(
        "Bound to address {} on port: {}",
        configuration.application.host,
        configuration.application.port
    );

    startup::run(listener, connection_pool, email_client)?.await
}
