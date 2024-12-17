use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use dotenv::dotenv;
use lettre::message::Message;
use lettre::{ Transport, SmtpTransport };
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;
use chrono::NaiveDate;

#[tokio::main]
async fn main() {
    dotenv().ok(); // loads variables from .env file

    // Retrieve credentials
    let email_user: String = env::var("EMAIL_USER").expect("EMAIL_USER is not set");
    let email_password: String = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD is not set");
    let smtp_server: String = env::var("SMTP_SERVER").expect("SMTP_SERVER is not set");
    let subject = "Test Email from Rust";
    let body = "Hello this is a test email from the Rust";

    let mailer = SmtpTransport::relay(&*smtp_server)
        .unwrap()
        .credentials(Credentials::from((email_user.clone(), email_password)))  // Add the credentials
        .build();

    let file_path = "recipients.txt";
    if let Ok(file) = File::open(file_path) {
        let reader: BufReader<File> = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let parts: Vec<&str> = line.split(',').collect();

                    if parts.len() != 4 {
                        // @todo: log
                        eprintln!("Invalid line received");
                    }
                    let id = parts[0].to_string();
                    let name = parts[1].to_string();
                    let email = parts[2].to_string();
                    let last_sent_date = parts[3].to_string();

                    let last_sent_date =
                        NaiveDate::parse_from_str(last_sent_date.as_str(), "%Y-%m-%d").unwrap();

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

                },
                Err(e) => {
                    // @todo: log
                    eprintln!("Error reading line: {:?}", e);
                    continue;
                }
            }
        }
    } else {
        // @todo: log
        eprintln!("Couldn't open file {}", file_path);
    }
}
