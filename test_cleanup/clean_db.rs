use sqlx::{Connection, PgConnection, Row};
use zero2prod::configuration::{DatabaseSettings, get_configuration};

const DB_PREFIX: &str = "test_newsletter_";

pub async fn delete_test_databases(config: &DatabaseSettings) {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    let databases = sqlx::query(
        format!(
            "SELECT datname FROM pg_database WHERE datname like '{}%';",
            DB_PREFIX
        )
        .as_str(),
    )
    .fetch_all(&mut connection)
    .await
    .expect("Trouble connecting to find existing test databases.");

    for db in databases {
        println!("db: {:?}\n", db);
        let db = db.get::<String, _>("datname");
        assert!(db.starts_with(DB_PREFIX));
        let drop_query = format!("drop database \"{}\";", db);
        let _ = sqlx::query(&drop_query).execute(&mut connection).await;
    }
}

#[tokio::main]
pub async fn main() {
    let config = {
        let mut c = get_configuration().expect("Failed to read configuration.");

        c.database.database_name =
            format!("{}{}", DB_PREFIX, "-not-used").to_string();

        c.application.port = 0;
        c
    };

    delete_test_databases(&config.database).await
}
