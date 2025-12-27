//! Runner extension support with PTY.

use crate::error::{ExtensionError, Result};
use opencmd_protocol::RunnerConfig;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::Read;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

/// A running PTY session for a runner extension.
pub struct RunnerSession {
    /// Unique session ID
    pub id: String,
    /// The runner configuration
    pub config: RunnerConfig,
    /// PTY master for reading/writing
    master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    /// Writer for sending input
    writer: Arc<Mutex<Box<dyn std::io::Write + Send>>>,
    /// Child process handle
    child: Arc<Mutex<Box<dyn portable_pty::Child + Send + Sync>>>,
}

impl RunnerSession {
    /// Spawn a new runner session.
    pub fn spawn(config: &RunnerConfig, input: Option<&str>) -> Result<Self> {
        let pty_system = NativePtySystem::default();

        // Create PTY pair
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;

        // Build command with argument substitution
        let mut cmd = CommandBuilder::new(&config.command);
        for arg in &config.args {
            let processed = if let Some(input) = input {
                arg.replace("{{prompt}}", input)
            } else {
                arg.replace("{{prompt}}", "")
            };
            cmd.arg(processed);
        }

        info!("Spawning runner: {} {:?}", config.command, config.args);

        // Spawn the child process
        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;

        // Get writer from master
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;

        let session_id = Uuid::new_v4().to_string();

        Ok(Self {
            id: session_id,
            config: config.clone(),
            master: Arc::new(Mutex::new(pair.master)),
            writer: Arc::new(Mutex::new(writer)),
            child: Arc::new(Mutex::new(child)),
        })
    }

    /// Write input to the PTY.
    pub async fn write(&self, data: &[u8]) -> Result<()> {
        use std::io::Write;
        let mut writer = self.writer.lock().await;
        writer
            .write_all(data)
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;
        writer
            .flush()
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;
        Ok(())
    }

    /// Read output from the PTY (non-blocking, returns available data).
    pub async fn read(&self) -> Result<Vec<u8>> {
        let master = self.master.lock().await;
        let mut reader = master
            .try_clone_reader()
            .map_err(|e| ExtensionError::Pty(e.to_string()))?;

        let mut buffer = vec![0u8; 4096];
        match reader.read(&mut buffer) {
            Ok(n) => {
                buffer.truncate(n);
                Ok(buffer)
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(vec![]),
            Err(e) => Err(ExtensionError::Io(e)),
        }
    }

    /// Resize the PTY.
    pub async fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        let master = self.master.lock().await;
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| ExtensionError::Pty(e.to_string()))
    }

    /// Check if the process has exited.
    pub async fn try_wait(&self) -> Result<Option<i32>> {
        let mut child = self.child.lock().await;
        match child.try_wait() {
            Ok(Some(status)) => {
                // exit_code returns u32, convert to i32
                Ok(Some(status.exit_code() as i32))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ExtensionError::Pty(e.to_string())),
        }
    }

    /// Kill the process.
    pub async fn kill(&self) -> Result<()> {
        let mut child = self.child.lock().await;
        child
            .kill()
            .map_err(|e| ExtensionError::Pty(e.to_string()))
    }
}
