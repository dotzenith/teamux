use anyhow::{Result, anyhow};
use std::process::Command;

#[derive(Debug)]
pub struct TmuxManager;

impl TmuxManager {
    pub fn new() -> Result<Self> {
        if let Err(_) = Command::new("tmux").arg("-V").output() {
            return Err(anyhow!("Unable to call tmux, please ensure it is correctly installed"));
        };

        if let Err(_) = Command::new("fzf").arg("--version").output() {
            return Err(anyhow!("Unable to call fzf, please ensure it is correctly installed"));
        };

        Ok(TmuxManager)
    }

    pub fn ls(&self) -> Result<Vec<String>> {
        let sessions = match Command::new("tmux").arg("list-sessions").output() {
            Ok(out) => {
                if !out.status.success() {
                    return Err(anyhow!("Unable to get sessions"));
                }
                String::from_utf8(out.stdout)?
            }
            Err(err) => {
                return Err(anyhow!(
                    "Unable to call tmux, please ensure it is correctly installed: {err}"
                ));
            }
        };

        sessions
            .lines()
            .map(|line| {
                line.split(":")
                    .nth(0)
                    .ok_or(anyhow!("Unable to parse sessions"))
                    .and_then(|sesh| Ok(sesh.to_string()))
            })
            .collect()
    }

    pub fn has_session(&self, session: &str) -> Result<bool> {
        match Command::new("tmux").arg("has-session").arg("-t").arg(session).status() {
            Ok(out) => Ok(out.success()),
            Err(err) => Err(anyhow!(
                "Unable to call tmux, please ensure it is correctly installed: {err}"
            )),
        }
    }
}
