use reqwest::Client;
use secrecy::{ExposeSecret, SecretBox};

use crate::domain::SubscriberEmail;

#[allow(dead_code)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: SecretBox<String>,
}

impl EmailClient {
    pub async fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretBox<String>,
    ) -> Self {
        EmailClient {
            http_client: reqwest::Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    #[allow(unused_variables)]
    pub async fn send_email(
        &self,
        receiver: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);

        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: receiver.as_ref().to_owned(),
            subject: subject.to_owned(),
            text_body: text_content.to_owned(),
            html_body: html_content.to_owned(),
        };

        let builder = self
            .http_client
            .post(&url)
            .json(&request_body)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .send()
            .await?;
        println!("builder = {:?}", builder);
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
pub struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    text_body: String,
    html_body: String,
}

#[cfg(test)]
mod test {
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::domain::SubscriberEmail;

    #[tokio::test]
    pub async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            SecretBox::new(Box::new("dog".to_string())),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .await
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
    }
}
