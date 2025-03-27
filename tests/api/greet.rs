use crate::helpers::spawn_app;

#[tokio::test]
async fn greet_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(app.address + "/tester")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(13), response.content_length());
    let text = response.text().await.expect("Failed to get response.");
    assert_eq!(text, "Hello tester!");
}
