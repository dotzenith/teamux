mod tmux;

use anyhow::Result;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let manager = tmux::TmuxManager::new().unwrap();

    let sessions = manager.ls().unwrap().join("\n");

    let mut fzf = Command::new("fzf")
        .arg("--height")
        .arg("40%")
        .arg("--reverse")
        .arg("--print-query")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = fzf.stdin.take() {
        stdin.write_all(sessions.as_bytes())?;
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
