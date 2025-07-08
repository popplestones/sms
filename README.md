# sms-cli

A command-line tool for sending SMS messages using providers like Twilio or 5CentSMS. It reads phone numbers from `stdin`, opens Neovim (or your `$EDITOR`) to compose a message, and sends the SMS upon saving and exiting.

---

## 📦 Installation

```bash
git clone https://github.com/popplestones/sms.git
cd sms
cargo build --release
```
Copy the binary somewhere in your `$PATH`, eg.

```bash
cp target/release/sms ~/.cargo/bin/
```

## 🚀 Usage

```bash
rolodex | jq -r .phone | sms
```

This will:
1. Open your `$EDITOR` (defaults to `nvim`) to enter an SMS message
2. Sendthe message to each phone number from stdin

## ⚙️ Configuration

The provider is selected via the `SMS_PROVIDER` environment variable. Supported values: `twilio`, `fivecent`

### Twilio

```bash
export SMS_PROVIDER=twilio
export TWILIO_ACCOUNT_SID=ACxxxxxxxxxxxxxxxxxxxxxxxx
export TWILIO_AUTH_TOKEN=your_auth_token
export TWILIO_FROM_NUMBER=+614xxxxxxxx
```

### 5 Cent SMS

```bash
export SMS_PROVIDER=fivecent
export FIVE_CENT_USERNAME=your_username
export FIVE_CENT_API_KEY=your_key
export FIVE_CENT_FROM_NUMBER=your_sender_id
```

## 🧪 Example

```bash
echo "+61412345678" | sms
```

## 📝 Notes
- Lines starting with `#` in the editor is ignored.
- If no message is entered, the operation is aborted.
- You can switch between providers by setting a different `SMS_PROVIDER`


## 🛡️ License
MIT
