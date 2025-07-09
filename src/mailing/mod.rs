use crate::data::user::User;
use askama::Template;
use lettre::{message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials, Address, Message, SmtpTransport, Transport};
use std::env;
use std::str::FromStr;

#[derive(Debug)]
pub enum EmailError {
    Smtp {
        smtp_err: lettre::transport::smtp::Error,
    },
    Mail {
        mail_err: lettre::error::Error,
    },
    Template {
        tmpl_err: askama::Error,
    },
}

pub struct EmailValidationRequest<'u> {
    validation_code: String,
    user: &'u User,
}

impl<'u> EmailValidationRequest<'u> {
    pub fn new(validation_code: String, user: &'u User) -> Self {
        Self {
            validation_code,
            user,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "email_validation.html")]
struct EmailValidationTemplate<'u> {
    username: &'u String,
    validation_link: String,
}

impl<'u> From<EmailValidationRequest<'u>> for EmailValidationTemplate<'u> {
    fn from(value: EmailValidationRequest<'u>) -> Self {
        let host = env::var("HOSTNAME").unwrap_or("localhost".to_string());
        let link = format!("{}/validate?code={}", &host, value.validation_code);
        Self {
            username: &value.user.username,
            validation_link: link,
        }
    }
}

pub struct EmailService {
    mailbox_from: Mailbox,
    smtp: SmtpTransport,
}

impl EmailService {
    pub fn new(smtp_host: &str, credentials: Credentials) -> Result<EmailService, EmailError> {
        let smtp = SmtpTransport::relay(smtp_host)?
            .credentials(credentials).build();
        let mailbox_from= Mailbox::new(Some("Farme.rs".to_string()), "".parse().expect("Could not parse email from"));
        Ok(Self {
            smtp,
            mailbox_from,
        })
    }

    pub fn send_validation_request(&self, request: EmailValidationRequest) -> Result<(), EmailError> {
        let recipient = Mailbox::new(Some(request.user.username.clone()), Address::from_str(&request.user.email).unwrap());
        let template: EmailValidationTemplate = request.into();
        let contents = template.render()?;
        let subject = "Farme.rs: Validate Email".to_string();
        self.send_mail(recipient, subject, contents, ContentType::TEXT_HTML)
    }

    fn send_mail(&self, recipient: Mailbox, subject: String, text: String, content_type: ContentType) -> Result<(), EmailError> {
        let email =  Message::builder()
            .from(self.mailbox_from.clone())
            .to(recipient)
            .subject(subject)
            .header(content_type)
            .body(text)?;
        self.smtp.send(&email)?;
        Ok(())
    }
}

pub fn mailer_from_env() -> Result<EmailService, EmailError> {
    let host = env::var("EMAIL_SMTP_HOST").expect("EMAIL_SMTP_HOST must be set");
    let user = env::var("EMAIL_USER").expect("EMAIL_USER must be set");
    let token = env::var("EMAIL_TOKEN").expect("EMAIL_TOKEN must be set");
    let credentials = Credentials::new(user, token);
    EmailService::new(&host, credentials)
}

impl From<lettre::transport::smtp::Error> for EmailError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        EmailError::Smtp {
            smtp_err: error,
        }
    }
}

impl From<lettre::error::Error> for EmailError {
    fn from(error: lettre::error::Error) -> Self {
        EmailError::Mail {
            mail_err: error,
        }
    }
}

impl From<askama::Error> for EmailError {
    fn from(value: askama::Error) -> Self {
        EmailError::Template {
            tmpl_err: value,
        }
    }
}