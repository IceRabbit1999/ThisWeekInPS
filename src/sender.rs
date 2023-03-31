use crate::config::Config;
use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::{ContentType, To};
use lettre::message::{header, Mailboxes};
use lettre::transport::smtp::authentication::Credentials;

pub fn send(config: &Config) -> Result<()> {
    let mail = &config.dev.mail;
    let mut to_address = String::new();
    for (index, address) in mail.to.iter().enumerate() {
        to_address.push_str(address);
        if index != mail.to.len() - 1 {
            to_address.push_str(", ");
        }
    }
    let mailboxes = to_address.parse::<Mailboxes>().unwrap();
    let to_header: To = mailboxes.into();

    let msg = Message::builder()
        .mailbox(to_header)
        .from(mail.from.parse()?)
        .subject("Test")
        .header(ContentType::TEXT_PLAIN)
        .body("test".to_string())?;

    let cred = Credentials::new(mail.username.clone(), mail.password.clone());
    let sender = SmtpTransport::relay(&mail.host)?
        .credentials(cred)
        .build();

    sender.send(&msg)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_config;

    #[test]
    fn msg() {
        let config = read_config();
        if send(&config).is_ok() {
            println!("Send Email success!")
        }else {
            println!(":(")
        }
    }
}