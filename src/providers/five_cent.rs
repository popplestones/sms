use std::collections::HashMap;

pub struct FiveCentClient {
    base_url: String,
    pub username: String,
    pub api_key: String,
    pub from_number: String,
}

impl FiveCentClient {
    pub fn new(username: String, api_key: String, from_number: String) -> Self {
        Self {
            base_url: "https://www.5centsms.com.au/api/v4/sms".to_string(),
            username,
            api_key,
            from_number,
        }
    }

    pub async fn send_sms(&self, to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut payload = HashMap::new();
        payload.insert("to", to);
        payload.insert("sender", self.from_number.as_str());
        payload.insert("message", body);

        let client = reqwest::Client::new();
        let res = client
            .post(&self.base_url)
            .header("User", &self.username)
            .header("Api-Key", &self.api_key)
            .form(&payload)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        if !status.is_success() {
            return Err(format!("5CentSMS error: {status} - {text}").into());
        }

        Ok(())
    }
}
