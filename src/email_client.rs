use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, authorization_token: Secret<String>) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
        is_transactional: bool,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v2/email/send", self.base_url);
        let request_body = SendEmailRequest {
            apikey: self.authorization_token.expose_secret(),
            subject,
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            body_html: html_content,
            body_text: text_content,
            is_transactional,
        };
        self.http_client.post(&url).json(&request_body).send().await?;
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailRequest<'a> {
    apikey: &'a str,
    subject: &'a str,
    from: &'a str,
    to: &'a str,
    body_html: &'a str,
    body_text: &'a str,
    is_transactional: bool,
}

#[cfg(test)]
mod tests {
    use fake::{faker::{internet::en::SafeEmail, lorem::en::{Paragraph, Sentence}}, Fake, Faker};
    use secrecy::Secret;
    use wiremock::{matchers::{header, method, path}, Mock, MockServer, ResponseTemplate};

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    // note from self - i hate this even more lmao
    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("from").is_some()
                    && body.get("to").is_some()
                    && body.get("subject").is_some()
                    && body.get("bodyHtml").is_some()
                    && body.get("bodyText").is_some()
                    && body.get("apikey").is_some()
                    && body.get("isTransactional").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        // note from self - I don't like mocks, just make thin HTTP clients, test the message format,
        // not the fact that you're sending an HTTP request, it should be obvious from the code;
        // It's probably a valid use-case for an integration test tho.
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

        Mock::given(SendEmailBodyMatcher)
            .and(header("Content-Type", "application/json"))
            .and(path("/v2/email/send"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content, true)
            .await;

        // Assert
        // Mock expectations are checked on drop
    }
}
