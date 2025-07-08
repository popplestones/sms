use async_trait::async_trait;

use crate::providers::{five_cent::FiveCentClient, twilio::TwilioClient};

pub mod five_cent;
pub mod twilio;

#[async_trait]
pub trait SmsProvider: Send + Sync {
    async fn send_sms(&self, to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub enum SmsClient {
    Twilio(TwilioClient),
    FiveCent(FiveCentClient),
}

impl SmsClient {
    pub fn from_env(provider: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match provider {
            "twilio" => {
                let sid = std::env::var("TWILIO_ACCOUNT_SID")
                    .map_err(|_| "Missing environment variable: TWILIO_ACCOUNT_SID")?;
                let token = std::env::var("TWILIO_AUTH_TOKEN")
                    .map_err(|_| "Missing environment variable: TWILIO_AUTH_TOKEN")?;
                let from = std::env::var("TWILIO_FROM_NUMBER")
                    .map_err(|_| "Missing environment variable: TWILIO_FROM_NUMBER")?;

                Ok(SmsClient::Twilio(TwilioClient::new(sid, token, from)))
            }
            "fivecent" => {
                let username = std::env::var("FIVE_CENT_USERNAME")
                    .map_err(|_| "Missing environment variable: FIVE_CENT_USERNAME")?;
                let api_key = std::env::var("FIVE_CENT_API_KEY")
                    .map_err(|_| "Missing environment variable: FIVE_CENT_API_KEY")?;
                let from = std::env::var("FIVE_CENT_FROM_NUMBER")
                    .map_err(|_| "Missing environment variable: FIVE_CENT_FROM_NUMBER")?;

                Ok(SmsClient::FiveCent(FiveCentClient::new(
                    username, api_key, from,
                )))
            }
            _ => Err(format!("Unsupported provider: {provider}").into()),
        }
    }
}

#[async_trait]
impl SmsProvider for SmsClient {
    async fn send_sms(&self, to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SmsClient::Twilio(client) => client.send_sms(to, body).await,
            SmsClient::FiveCent(client) => client.send_sms(to, body).await,
        }
    }
}
