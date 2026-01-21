use anyhow::{Context, Result};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::config::SmtpConfig;
use crate::notifier::Notifier;

#[derive(Clone)]
pub struct SmtpNotifier {
    config: SmtpConfig,
}

impl SmtpNotifier {
    pub fn new(config: SmtpConfig) -> Self {
        Self { config }
    }
}

impl Notifier for SmtpNotifier {
    fn send(&self, title: &str, message: &str) -> Result<()> {
        let subject = self
            .config
            .subject
            .clone()
            .unwrap_or_else(|| title.to_string());

        let email = Message::builder()
            .from(self.config.from.parse::<Mailbox>()?)
            .to(self.config.to.parse::<Mailbox>()?)
            .subject(subject)
            .body(format!("{}\n\n{}", title, message))
            .context("failed to build email message")?;

        let creds = Credentials::new(
            self.config.username.clone(),
            self.config.password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.config.host)
            .with_context(|| format!("invalid SMTP host {}", self.config.host))?
            .port(self.config.port)
            .credentials(creds)
            .build();

        mailer.send(&email).context("failed to send smtp message")?;
        Ok(())
    }
}
