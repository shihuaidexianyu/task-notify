use anyhow::{Context, Result};
use std::process::Command;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

#[derive(Debug)]
pub enum MonitorType {
    Command(Vec<String>),
    Pid(u32),
}

#[derive(Debug)]
pub struct RunOutcome {
    pub success: bool,
    pub message: String,
}

pub fn run_command(cmd: &[String]) -> Result<RunOutcome> {
    let start = Instant::now();
    let status = Command::new(&cmd[0])
        .args(&cmd[1..])
        .status()
        .with_context(|| format!("failed to run command: {}", cmd[0]))?;
    let duration = start.elapsed();
    let message = format!(
        "Command '{}' finished in {:.2}s with status: {}",
        cmd[0],
        duration.as_secs_f32(),
        status
    );
    Ok(RunOutcome {
        success: status.success(),
        message,
    })
}

pub fn watch_pid(pid_num: u32) -> Result<String> {
    let mut sys = System::new_all();
    let pid = Pid::from(pid_num as usize);
    sys.refresh_processes();

    if sys.process(pid).is_none() {
        anyhow::bail!("PID {} not found", pid_num);
    }

    loop {
        std::thread::sleep(Duration::from_secs(2));
        sys.refresh_processes();
        if sys.process(pid).is_none() {
            break;
        }
    }

    Ok(format!("Process {} has exited.", pid_num))
}
