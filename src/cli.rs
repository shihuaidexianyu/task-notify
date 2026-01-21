use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::monitor::MonitorType;

#[derive(Parser, Debug)]
#[command(name = "task-notify", version, about = "Send an SMTP notification when a task finishes.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(long, value_name = "MESSAGE", global = true)]
    pub msg: Option<String>,

    #[arg(long, global = true)]
    pub silent: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run {
        #[arg(trailing_var_arg = true, required = true)]
        cmd: Vec<String>,
    },
    Watch {
        pid: u32,
    },
}

#[derive(Debug)]
pub struct RuntimeArgs {
    pub monitor_type: MonitorType,
    pub custom_message: Option<String>,
    pub silent: bool,
}

impl RuntimeArgs {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        let monitor_type = match &cli.command {
            Commands::Run { cmd } => MonitorType::Command(cmd.clone()),
            Commands::Watch { pid } => MonitorType::Pid(*pid),
        };

        Ok(Self {
            monitor_type,
            custom_message: cli.msg.clone(),
            silent: cli.silent,
        })
    }
}
