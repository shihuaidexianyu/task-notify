use anyhow::Result;
use chrono::Local;
use clap::Parser;

mod cli;
mod config;
mod monitor;
mod notifier;

use cli::{Cli, RuntimeArgs};
use config::load_config;
use monitor::{run_command, watch_pid, MonitorType};
use notifier::smtp::SmtpNotifier;
use notifier::Notifier;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let args = RuntimeArgs::from_cli(&cli)?;
    let config = load_config(cli.config.as_deref())?;

    let notifier = SmtpNotifier::new(config.smtp.clone());

    let (result_msg, success) = match args.monitor_type {
        MonitorType::Command(cmd) => {
            let outcome = run_command(&cmd)?;
            if args.silent && outcome.success {
                return Ok(());
            }
            (outcome.message, outcome.success)
        }
        MonitorType::Pid(pid) => (watch_pid(pid)?, true),
    };

    let final_msg = args
        .custom_message
        .or_else(|| {
            if config.default_message.is_empty() {
                None
            } else {
                Some(config.default_message)
            }
        })
        .unwrap_or(result_msg);

    let title = if success {
        "Task finished"
    } else {
        "Task failed"
    };

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let message = format!("{} ({})", final_msg, timestamp);

    println!("Sending notification...");
    notifier.send(title, &message)?;

    Ok(())
}
