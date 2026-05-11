use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub language: String,
    pub code: String,
    #[serde(default)]
    pub stdin: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteResponse {
    pub language: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub duration_ms: u128,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize)]
pub struct LanguagesResponse {
    pub languages: Vec<LanguageInfo>,
}

#[derive(Debug, Serialize)]
pub struct LanguageInfo {
    pub id: &'static str,
    pub aliases: &'static [&'static str],
}
