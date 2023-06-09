use once_cell::sync::Lazy;
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    email_client::EmailClient,
    telemetry::{get_subscriber, init_subscriber},
};

pub struct TestApp {
    pub addr: String,
    pub client: Client,
    pub db_pool: PgPool,
}

pub static TRACING: Lazy<()> = Lazy::new(|| {
    let (name, level) = (String::from("zero2prod"), String::from("debug"));

    if std::env::var("TEST_LOG").is_ok() {
        let sub = get_subscriber(name, level, std::io::stdout);
        init_subscriber(sub);
        return;
    }

    let sub = get_subscriber(name, level, std::io::stdout);
    init_subscriber(sub);
});

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to database");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create db");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to apply migrations");

    pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listnener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    let port = listnener.local_addr().unwrap().port();
    let mut config = get_configuration().expect("Failed to parse config");
    config.database.database_name = uuid::Uuid::new_v4().to_string();

    let db_pool = configure_db(&config.database).await;
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

    let srv = zero2prod::startup::run(listnener, db_pool.clone(), email_client)
        .await
        .expect("Failed to spawn our app.");
    let addr = format!("http://127.0.0.1:{}", port);

    tokio::spawn(srv);

    TestApp {
        addr,
        client: reqwest::Client::new(),
        db_pool,
    }
}
