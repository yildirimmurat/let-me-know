use std::env;
use dotenv::dotenv;
use lettre::message::Message;
use lettre::{ Transport, SmtpTransport };
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;

fn main() {
    dotenv().ok(); // loads variables from .env file

    // Retrieve credentials
    let email_user: String = env::var("EMAIL_USER").expect("EMAIL_USER is not set");
    let email_password: String = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD is not set");
    let recipient_email: String = env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL is not set");
    let smtp_server: String = env::var("SMTP_SERVER").expect("SMTP_SERVER is not set");

    let subject = "Test Email from Rust";
    let body = "Hello this is a test email from the Rust";
    let email = Message::builder()
        .from(email_user.parse().unwrap())
        .to(recipient_email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_owned())
        .unwrap();

    let credentials = Credentials::new(email_user, email_password);

    let mailer = SmtpTransport::relay(&*smtp_server)
        .unwrap()
        .credentials(credentials)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(_) => panic!("Could not send email"),
    }
}
