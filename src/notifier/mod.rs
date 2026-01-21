use anyhow::Result;

pub mod smtp;

pub trait Notifier {
    fn send(&self, title: &str, message: &str) -> Result<()>;
}
