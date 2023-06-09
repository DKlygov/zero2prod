use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let res = app
        .client
        .get(&format!("{}/health_check", &app.addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}
