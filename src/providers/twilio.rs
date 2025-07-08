#[derive(Debug)]
pub struct TwilioClient {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
}

impl TwilioClient {
    pub fn new(account_sid: String, auth_token: String, from_number: String) -> Self {
        Self {
            account_sid,
            auth_token,
            from_number,
        }
    }
    pub async fn send_sms(&self, to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );
        let params = [
            ("From", self.from_number.as_str()),
            ("To", to),
            ("Body", body),
        ];

        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await?;
            return Err(format!("Twilio API error: {}", text).into());
        }
        Ok(())
    }
}
