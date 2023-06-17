use sqlx::postgres::PgPoolOptions;
use std::{io::stdout, net::TcpListener};

use zero2prod::{
    configuration::get_configuration, email_client::EmailClient, startup::run, telemetry::*,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let sub = get_subscriber(String::from("zero2prod"), String::from("info"), stdout);
    init_subscriber(sub);

    let config = get_configuration().expect("Failed to parse config");
    let conn = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_lazy_with(config.database.with_db());
    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = config.email_client.timeout();

    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.authorization_token,
        timeout,
    );

    let addr = format!("{}:{}", config.application.host, config.application.port);

    run(
        TcpListener::bind(addr).expect("Failed to bind port 8080"),
        conn,
        email_client,
    )
    .await?
    .await
}
