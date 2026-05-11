pub mod docker;

use std::time::Duration;

use crate::{error::AppError, runtimes::RuntimeSpec};

#[derive(Debug)]
pub struct SandboxRequest<'a> {
    pub runtime: RuntimeSpec,
    pub code: &'a str,
    pub stdin: &'a str,
    pub timeout: Duration,
}

#[derive(Debug)]
pub struct SandboxOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub duration_ms: u128,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

pub trait SandboxExecutor {
    async fn execute(&self, request: SandboxRequest<'_>) -> Result<SandboxOutput, AppError>;
}
