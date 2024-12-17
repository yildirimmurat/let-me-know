use std::env;
use dotenv::dotenv;
use lettre::message::Message;
use lettre::{ Transport, SmtpTransport };
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;

mod db;
mod user_db;

#[tokio::main]
async fn main() {
    dotenv().ok(); // loads variables from .env file

    // Retrieve credentials
    let email_user: String = env::var("EMAIL_USER").expect("EMAIL_USER is not set");
    let email_password: String = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD is not set");
    let smtp_server: String = env::var("SMTP_SERVER").expect("SMTP_SERVER is not set");
    let subject = "Test Email from Rust";
    let body = "Hello this is a test email from the Rust";

    let conn = db::setup_db().expect("Could not setup database");
    let users = user_db::get_all_users(&conn).expect("Could not get users");

    let mailer = SmtpTransport::relay(&*smtp_server)
        .unwrap()
        .credentials(Credentials::from((email_user.clone(), email_password)))  // Add the credentials
        .build();

    for (name, email) in users {
        let email_msg = Message::builder()
            .from(email_user.parse().unwrap())
            .to(email.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_owned())
            .unwrap();

        match mailer.send(&email_msg) {
            Ok(_) => {
                println!("Email sent to : {} on address {}", name, email);
            },
            Err(e) => {
                // @todo: log
                eprintln!("Failed to send email to {}: {}", name, e);
            }
        }
    }
}
