use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecuteRequest {
    pub language: String,
    pub code: String,
    #[serde(default)]
    pub stdin: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LanguagesResponse {
    pub languages: Vec<LanguageInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LanguageInfo {
    pub id: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
}
