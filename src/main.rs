use std::{env, io};

use sms::providers::{SmsClient, SmsProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let phones = sms::read_phone_numbers(io::stdin().lock());

    if phones.is_empty() {
        eprintln!("No phone numbers provided.");
        std::process::exit(1);
    }

    let message = sms::open_editor_with_template("")?;
    if message.is_empty() {
        eprintln!("No message entered. Aborting.");
        std::process::exit(1);
    }

    let provider = env::var("SMS_PROVIDER").unwrap_or_else(|_| "fivecent".into());

    let sms_client = SmsClient::from_env(&provider)?;

    for phone in phones {
        sms_client.send_sms(&phone, &message).await?;
    }

    Ok(())
}
