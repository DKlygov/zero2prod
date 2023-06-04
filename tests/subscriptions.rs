mod common;
use crate::common::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let res = app
        .client
        .post(&format!("{}/subscriptions", &app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("name=le%20guin", "missing the email"),
        ("", "missing both name and email"),
    ];

    for (body, msg) in test_cases {
        let res = app
            .client
            .post(&format!("{}/subscriptions", &app.addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            res.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            msg,
        );
    }
}
