mod tmux;

use anyhow::Result;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser)]
#[command(name = "mux")]
#[command(about, version, author)]

struct Cli {
    /// Name of the session mux will attach to or create
    name: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manager = tmux::TmuxManager::new()?;

    match cli.name.as_deref() {
        Some(name) => {
            if manager.has_session(name)? {
                let err = Command::new("tmux").arg("attach").arg("-t").arg(name).exec();
                return Err(err.into());
            } else {
                let err = Command::new("tmux").arg("new").arg("-s").arg(name).exec();
                return Err(err.into());
            }
        }
        None => (),
    }

    let sessions = manager.ls()?;

    if sessions.is_empty() {
        print!("New Session: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let err = Command::new("tmux").arg("new").arg("-s").arg(input.trim()).exec();
        return Err(err.into());
    }

    let mut fzf = Command::new("fzf")
        .arg("--height")
        .arg("40%")
        .arg("--reverse")
        .arg("--print-query")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = fzf.stdin.take() {
        stdin.write_all(sessions.join("\n").as_bytes())?;
    }

    let output = fzf.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    match lines.as_slice() {
        [] => Ok(()),
        [query] => {
            let err = Command::new("tmux").arg("new").arg("-s").arg(query).exec();
            Err(err.into())
        }
        [_, selection, ..] => {
            let err = Command::new("tmux").arg("attach").arg("-t").arg(selection).exec();
            Err(err.into())
        }
    }
}
