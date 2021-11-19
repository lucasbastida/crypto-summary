use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MailerConfig {
    username: String,
    pw: String,
}

pub fn create_email(portfolio_msg: String) -> Message {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to("Hei <test@example.com>".parse().unwrap())
        .subject("Happy new year")
        .body(portfolio_msg)
        .unwrap();
    return email;
}

pub fn send_email(email: Message) -> () {
    dotenv::dotenv().expect("Failed to read .env file");
    let config = match envy::prefixed("EMAIL_SMTP_").from_env::<MailerConfig>() {
        Ok(config) => {
            println!("{:?}", config);
            config
        }
        Err(e) => panic!("Couldn't read mailer config ({})", e),
    };

    let creds = Credentials::new(config.username, config.pw);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    };
}
