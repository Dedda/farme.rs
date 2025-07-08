use crate::data::user::User;
use handlebars::{Handlebars, RenderError};
use lazy_static::lazy_static;
use lettre::{message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials, Address, Message, SmtpTransport, Transport};
use serde_json::json;
use std::env;
use std::str::FromStr;

pub const EMAIL_VALIDATION: &str = "email-validation";

lazy_static! {
    static ref TEMPLATES: Handlebars<'static> = {
        let mut hbs = Handlebars::new();
        hbs.register_template_string(EMAIL_VALIDATION, include_str!("templates/email_validation.hbs.html")).expect("Failed to parse email-validation");
        hbs
    };
}

#[derive(Debug)]
pub enum EmailError {
    Smtp {
        smtp_err: lettre::transport::smtp::Error,
    },
    Mail {
        mail_err: lettre::error::Error,
    },
    Template {
        tmpl_err: RenderError,
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
        let host = env::var("HOSTNAME").unwrap_or("localhost".to_string());
        let link = format!("{}/validate?code={}", &host, request.validation_code);
        let ctx = json!({"username": &request.user.username, "validation_link": &link});
        let contents = TEMPLATES.render(EMAIL_VALIDATION, &ctx)?;
        let recipient = Mailbox::new(Some(request.user.username.clone()), Address::from_str(&request.user.email).unwrap());
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

impl From<RenderError> for EmailError {
    fn from(value: RenderError) -> Self {
        EmailError::Template {
            tmpl_err: value,
        }
    }
}