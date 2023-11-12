//use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, /*SmtpTransport, Transport*/};

use std::process::Command;

use crate::redacted_string::RedactedString;


#[derive(Debug)]
pub struct Config {
    pub confirm: String,
    pub from: String,
    pub smtpserver: String,
    pub smtpuser: String,
    pub smtpencryption: String,
    pub smtpserverport: String,
    pub smtppass: RedactedString,
}

pub fn build_email(sender_name: String, sender_email: String, to: String,
               subject: String, body: String) -> lettre::Message {
    Message::builder()
        .from(format!("{} <{}>", sender_name, sender_email).parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap()
}

fn git_config(setting: &str) -> String {
    let out_vec = Command::new("git")
            .args(["config", setting])
            .output()
            .expect("failed to execute `git config ...`")
            .stdout;

    String::from_utf8(out_vec)
            .expect("failed to convert `git config ...` output to string")
            .trim()
            .to_string()
}

pub fn load_config() -> Config {
    Config {
        confirm: git_config("sendemail.confirm"),
        from: git_config("sendemail.from"),
        smtpserver: git_config("sendemail.smtpserver"),
        smtpuser: git_config("sendemail.smtpuser"),
        smtpencryption: git_config("sendemail.smtpencryption"),
        smtpserverport: git_config("sendemail.smtpserverport"),
        smtppass: RedactedString(git_config("sendemail.smtppass")),
    }
}
