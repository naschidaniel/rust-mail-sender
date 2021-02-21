extern crate lettre;
extern crate lettre_email;

use dotenv;

use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

fn main() {
    let alias = dotenv::var("ALIAS").unwrap();
    let smtp_address = dotenv::var("SMTP_ADDRESS").unwrap();
    let smtp_username = dotenv::var("SMTP_USERNAME").unwrap();
    let smtp_password = dotenv::var("SMTP_PASSWORD").unwrap();
    let receiver = dotenv::var("RECEIVER").unwrap();

    let email_from_smtp_username = smtp_username.to_string();
    let email = EmailBuilder::new()
        .to((receiver, alias))
        .from(email_from_smtp_username)
        .subject("Hi, Hello world12121212121212")
        .text("Hello world.")
        .build()
        .unwrap()
        .into();
    
    let credentials = (&smtp_username, smtp_password).into_credentials();
    let mut client = SmtpClient::new_simple(&smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();

    let _result = client.send(email);

    if _result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", _result);
    }

    //assert!(_result.is_ok());
}
