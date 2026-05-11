use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use tokio::{
    fs,
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
    time,
};

use crate::{
    config::Config,
    error::AppError,
    sandbox::{SandboxExecutor, SandboxOutput, SandboxRequest},
};

pub struct DockerSandbox {
    config: Arc<Config>,
}

struct Workspace {
    path: PathBuf,
}

struct LimitedOutput {
    bytes: Vec<u8>,
    truncated: bool,
}

impl DockerSandbox {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    fn docker_command(
        &self,
        container_name: &str,
        workspace: &Path,
        request: &SandboxRequest<'_>,
    ) -> Command {
        let mut command = Command::new(&self.config.docker_bin);

        command
            .arg("run")
            .arg("--rm")
            .arg("--name")
            .arg(container_name)
            .arg("--interactive")
            .arg("--network")
            .arg("none")
            .arg("--cpus")
            .arg(&self.config.cpus)
            .arg("--memory")
            .arg(&self.config.memory)
            .arg("--memory-swap")
            .arg(&self.config.memory)
            .arg("--pids-limit")
            .arg(self.config.pids_limit.to_string())
            .arg("--read-only")
            .arg("--cap-drop")
            .arg("ALL")
            .arg("--security-opt")
            .arg("no-new-privileges")
            .arg("--user")
            .arg("65534:65534")
            .arg("--workdir")
            .arg("/workspace")
            .arg("--tmpfs")
            .arg("/tmp:rw,noexec,nosuid,nodev,size=64m")
            .arg("--mount")
            .arg(format!(
                "type=bind,source={},target=/workspace,readonly",
                workspace.display()
            ));

        for (key, value) in request.runtime.env {
            command.arg("--env").arg(format!("{key}={value}"));
        }

        command.arg(request.runtime.image);
        command.args(request.runtime.command);

        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        command
    }

    async fn force_remove(&self, container_name: &str) {
        let _ = Command::new(&self.config.docker_bin)
            .arg("rm")
            .arg("-f")
            .arg(container_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
    }
}

impl SandboxExecutor for DockerSandbox {
    async fn execute(&self, request: SandboxRequest<'_>) -> Result<SandboxOutput, AppError> {
        let started = Instant::now();
        let workspace = Workspace::create(&self.config.tmp_dir).await?;
        let source_path = workspace.path.join(request.runtime.file_name);
        fs::write(&source_path, request.code).await?;
        fs::set_permissions(&source_path, std::fs::Permissions::from_mode(0o444)).await?;

        let container_name = unique_name("sandbox-rs");
        let mut child = self
            .docker_command(&container_name, &workspace.path, &request)
            .spawn()
            .map_err(|err| AppError::Sandbox(format!("failed to start docker: {err}")))?;

        if let Some(mut stdin) = child.stdin.take() {
            let input = request.stdin.as_bytes().to_vec();
            tokio::spawn(async move {
                let _ = stdin.write_all(&input).await;
            });
        }

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| AppError::Sandbox("failed to capture stdout".to_owned()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| AppError::Sandbox("failed to capture stderr".to_owned()))?;

        let stdout_limit = self.config.max_output_bytes;
        let stderr_limit = self.config.max_output_bytes;
        let stdout_task = tokio::spawn(read_limited(stdout, stdout_limit));
        let stderr_task = tokio::spawn(read_limited(stderr, stderr_limit));

        let status = match time::timeout(request.timeout, child.wait()).await {
            Ok(status) => status?,
            Err(_) => {
                let _ = child.kill().await;
                self.force_remove(&container_name).await;

                let stdout = stdout_task
                    .await
                    .map_err(|err| AppError::Sandbox(format!("stdout task failed: {err}")))??;
                let stderr = stderr_task
                    .await
                    .map_err(|err| AppError::Sandbox(format!("stderr task failed: {err}")))??;

                return Ok(SandboxOutput {
                    stdout: stdout.bytes,
                    stderr: stderr.bytes,
                    exit_code: None,
                    timed_out: true,
                    duration_ms: started.elapsed().as_millis(),
                    stdout_truncated: stdout.truncated,
                    stderr_truncated: stderr.truncated,
                });
            }
        };

        let stdout = stdout_task
            .await
            .map_err(|err| AppError::Sandbox(format!("stdout task failed: {err}")))??;
        let stderr = stderr_task
            .await
            .map_err(|err| AppError::Sandbox(format!("stderr task failed: {err}")))??;

        Ok(SandboxOutput {
            stdout: stdout.bytes,
            stderr: stderr.bytes,
            exit_code: status.code(),
            timed_out: false,
            duration_ms: started.elapsed().as_millis(),
            stdout_truncated: stdout.truncated,
            stderr_truncated: stderr.truncated,
        })
    }
}

impl Workspace {
    async fn create(root: &Path) -> Result<Self, AppError> {
        let path = root.join(unique_name("sandbox-rs-work"));
        fs::create_dir(&path).await?;
        fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).await?;
        Ok(Self { path })
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

async fn read_limited<R>(mut reader: R, limit: usize) -> Result<LimitedOutput, std::io::Error>
where
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::new();
    let mut truncated = false;
    let mut chunk = [0_u8; 8192];

    loop {
        let read = reader.read(&mut chunk).await?;
        if read == 0 {
            break;
        }

        let remaining = limit.saturating_sub(bytes.len());
        if remaining > 0 {
            let keep = remaining.min(read);
            bytes.extend_from_slice(&chunk[..keep]);
        }
        if read > remaining {
            truncated = true;
        }
    }

    Ok(LimitedOutput { bytes, truncated })
}

fn unique_name(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}-{}-{nanos}", std::process::id())
}
