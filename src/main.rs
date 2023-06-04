use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::{io::stdout, net::TcpListener};

use zero2prod::{configuration::get_configuration, startup::run, telemetry::*};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let sub = get_subscriber(String::from("zero2prod"), String::from("info"), stdout);
    init_subscriber(sub);

    let config = get_configuration().expect("Failed to parse config");
    let conn = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    run(
        TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080"),
        conn,
    )
    .await?
    .await
}
