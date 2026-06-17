use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Arc;

use anyhow::{Context, Result};
use parking_lot::Mutex;

pub struct LlamaSidecar {
    process: Arc<Mutex<Option<Child>>>,
    port: u16,
}

impl LlamaSidecar {
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            port: 8080,
        }
    }

    pub fn start(&self, model_path: &std::path::Path) -> Result<()> {
        let binary = Self::find_binary()?;

        let child = Command::new(&binary)
            .arg("-m")
            .arg(model_path)
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--ctx-size")
            .arg("4096")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .context("failed to start llama-server")?;

        *self.process.lock() = Some(child);
        tracing::info!("llama-server started on port {}", self.port);
        Ok(())
    }

    pub fn stop(&self) {
        if let Some(mut child) = self.process.lock().take() {
            let _ = child.kill();
            let _ = child.wait();
            tracing::info!("llama-server stopped");
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    fn find_binary() -> Result<PathBuf> {
        let binary_name = if cfg!(target_os = "windows") {
            "llama-server.exe"
        } else {
            "llama-server"
        };

        let bundled = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_default()
            .join("llama")
            .join(binary_name);

        if bundled.exists() {
            return Ok(bundled);
        }

        which::which(binary_name).context("llama-server not found in PATH or bundled")
    }
}

impl Drop for LlamaSidecar {
    fn drop(&mut self) {
        self.stop();
    }
}
