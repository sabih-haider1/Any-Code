//! Native PTY sessions (PRD §82). This crate owns spawning and I/O only; the Tauri
//! layer owns the session table and turns bytes into frontend events, because "how a
//! session's lifecycle is tracked" is app-shell policy, not terminal mechanics.

use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error(transparent)]
    Pty(#[from] anyhow::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// The user's login shell, falling back to something that exists on every platform.
fn default_shell() -> String {
    if cfg!(windows) {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
    }
}

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
}

impl PtySession {
    /// Spawns the user's shell in `cwd`, returning the session handle plus a reader
    /// the caller owns directly — reading blocks, so it belongs on its own thread.
    pub fn spawn(
        cwd: &Path,
        cols: u16,
        rows: u16,
    ) -> Result<(Self, Box<dyn Read + Send>), TerminalError> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new(default_shell());
        cmd.cwd(cwd);

        let child = pair.slave.spawn_command(cmd)?;
        drop(pair.slave);

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        Ok((
            Self {
                master: pair.master,
                writer,
                child,
            },
            reader,
        ))
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), TerminalError> {
        self.writer.write_all(data)?;
        Ok(())
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<(), TerminalError> {
        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), TerminalError> {
        self.child.kill()?;
        Ok(())
    }
}
