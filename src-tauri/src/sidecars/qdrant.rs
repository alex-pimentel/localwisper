use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Arc;

use anyhow::{Context, Result};
use parking_lot::Mutex;

pub struct QdrantSidecar {
    process: Arc<Mutex<Option<Child>>>,
    data_dir: PathBuf,
    port: u16,
}

impl QdrantSidecar {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::cache_dir()
            .context("no cache dir")?
            .join("lightwisper")
            .join("qdrant-data");
        std::fs::create_dir_all(&data_dir)?;

        Ok(Self {
            process: Arc::new(Mutex::new(None)),
            data_dir,
            port: 6333,
        })
    }

    pub fn start(&self) -> Result<()> {
        let binary = Self::find_binary()?;
        let child = Command::new(&binary)
            .arg("--storage")
            .arg(&self.data_dir)
            .arg("--grpc-port")
            .arg((self.port + 1).to_string())
            .arg("--http-port")
            .arg(self.port.to_string())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .context("failed to start Qdrant")?;

        *self.process.lock() = Some(child);
        tracing::info!("qdrant started on port {}", self.port);
        Ok(())
    }

    pub fn stop(&self) {
        if let Some(mut child) = self.process.lock().take() {
            let _ = child.kill();
            let _ = child.wait();
            tracing::info!("qdrant stopped");
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    fn find_binary() -> Result<PathBuf> {
        let binary_name = if cfg!(target_os = "windows") {
            "qdrant.exe"
        } else {
            "qdrant"
        };

        // Check bundled binary first, then PATH
        let bundled = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_default()
            .join("qdrant")
            .join(binary_name);

        if bundled.exists() {
            return Ok(bundled);
        }

        // Check resources dir
        let resource = PathBuf::from("resources/bin/qdrant").join(binary_name);
        if resource.exists() {
            return Ok(resource);
        }

        which::which(binary_name).context("qdrant binary not found")
    }
}

impl Drop for QdrantSidecar {
    fn drop(&mut self) {
        self.stop();
    }
}
