# task-notify

Send an email notification via SMTP when a task finishes or a PID exits.

## Requirements

- Rust toolchain (stable)
- An SMTP account (host, port, username, password)

## Install

Build a release binary:

```bash
cargo build --release
```

The binary is at `target/release/task-notify`.

## Add to PATH (Linux/macOS)

If you want to run `task-notify` anywhere, add the binary directory to `PATH` or place the binary in a directory already in `PATH`. Common options:

### Option 1: Cargo install (recommended)

```bash
cargo install --path .
```

If `~/.cargo/bin` is in your `PATH`, you can run:

```bash
task-notify --help
```

### Option 2: Copy to `/usr/local/bin`

```bash
sudo cp ./target/release/task-notify /usr/local/bin/
```

### Option 3: Add a custom directory to `PATH`

```bash
mkdir -p ~/my-tools
cp ./target/release/task-notify ~/my-tools/
```

Add this line to your shell config (`~/.zshrc` or `~/.bashrc`):

```bash
export PATH="$HOME/my-tools:$PATH"
```

Reload your shell:

```bash
source ~/.zshrc
```

Verify:

```bash
which task-notify
```

## Configuration

Create a config file:

- Windows: `%APPDATA%\\task-notify.toml`
- Linux/macOS: `~/.config/task-notify.toml`

Example (`config.example.toml`):

```toml
default_message = "Task completed."

[smtp]
host = "smtp.example.com"
port = 587
username = "user@example.com"
password = "your-app-password"
from = "user@example.com"
to = "you@example.com"
subject = "Task Notify"
```

## Usage

Run a command and notify when it exits:

```bash
task-notify run -- cargo build --release
```

Watch an existing PID:

```bash
task-notify watch 1234
```

Override the message:

```bash
task-notify run --msg "Build finished" -- cargo build --release
```

Only notify on failure:

```bash
task-notify run --silent -- cargo build --release
```

Use a custom config file:

```bash
task-notify --config C:\\path\\to\\task-notify.toml run -- cargo build --release
```
