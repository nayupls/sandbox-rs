use std::{env, net::SocketAddr, path::PathBuf, time::Duration};

use crate::error::AppError;

#[derive(Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub docker_bin: String,
    pub tmp_dir: PathBuf,
    pub max_code_bytes: usize,
    pub max_stdin_bytes: usize,
    pub max_output_bytes: usize,
    pub default_timeout: Duration,
    pub max_timeout: Duration,
    pub memory: String,
    pub cpus: String,
    pub pids_limit: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            bind_addr: parse_env("SANDBOX_BIND_ADDR", "127.0.0.1:8080")?,
            docker_bin: string_env("SANDBOX_DOCKER_BIN", "docker"),
            tmp_dir: PathBuf::from(
                env::var("SANDBOX_TMP_DIR")
                    .unwrap_or_else(|_| env::temp_dir().to_string_lossy().into_owned()),
            ),
            max_code_bytes: parse_env("SANDBOX_MAX_CODE_BYTES", "65536")?,
            max_stdin_bytes: parse_env("SANDBOX_MAX_STDIN_BYTES", "65536")?,
            max_output_bytes: parse_env("SANDBOX_MAX_OUTPUT_BYTES", "65536")?,
            default_timeout: Duration::from_millis(parse_env(
                "SANDBOX_DEFAULT_TIMEOUT_MS",
                "3000",
            )?),
            max_timeout: Duration::from_millis(parse_env("SANDBOX_MAX_TIMEOUT_MS", "10000")?),
            memory: string_env("SANDBOX_MEMORY", "128m"),
            cpus: string_env("SANDBOX_CPUS", "0.5"),
            pids_limit: parse_env("SANDBOX_PIDS_LIMIT", "64")?,
        })
    }
}

fn string_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn parse_env<T>(key: &str, default: &str) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = env::var(key).unwrap_or_else(|_| default.to_owned());
    value
        .parse()
        .map_err(|err| AppError::Config(format!("{key}={value:?} is invalid: {err}")))
}
