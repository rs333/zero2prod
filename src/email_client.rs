use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

use crate::domain::SubscriberEmail;

#[allow(dead_code)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretString,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();

        EmailClient {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    #[allow(unused_variables)]
    pub async fn send_email(
        &self,
        receiver: &SubscriberEmail,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);

        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: receiver.as_ref(),
            subject,
            text_body,
            html_body,
        };

        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .header("Accept", "application/json")
            .send()
            .await?
            .error_for_status()?;
        println!("response = {:?}", response);
        // let response = r#"{
        //     "ErrorCode":0,
        //     "Message":"OK",
        //     "MessageID":"ababababababababababab",
        //     "SubmittedAt":"2025-03-27T23:15:33.175091Z",
        //     "To":"an_email@domain.com"
        //     }"#;
        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    text_body: &'a str,
    html_body: &'a str,
}

#[cfg(test)]
mod test {
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::domain::SubscriberEmail;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            match serde_json::from_slice::<serde_json::Value>(&request.body) {
                Ok(body) => {
                    body.get("From").is_some()
                        && body.get("To").is_some()
                        && body.get("Subject").is_some()
                        && body.get("HtmlBody").is_some()
                }
                Err(e) => {
                    println!("Error in body: {:?}", e);
                    false
                }
            }
        }
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(uri: String) -> EmailClient {
        EmailClient::new(
            uri,
            email(),
            SecretString::new(Faker.fake::<String>().into()),
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    pub async fn send_email_sends_the_expected_request() {
        let mock_server = MockServer::start().await;

        Mock::given(path("/email"))
            .and(method("POST"))
            .and(header("Accept", "application/json"))
            .and(header("Content-Type", "application/json"))
            .and(header_exists("X-Postmark-Server-Token"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = email_client(mock_server.uri())
            .send_email(&email(), &subject(), &content(), &content())
            .await;
    }

    #[tokio::test]
    pub async fn send_email_succeeds_if_the_server_returns_200() {
        let mock_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client(mock_server.uri())
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_ok!(outcome)
    }

    #[tokio::test]
    pub async fn send_email_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client(mock_server.uri())
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    pub async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(std::time::Duration::from_secs(11)),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client(mock_server.uri())
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }
}
