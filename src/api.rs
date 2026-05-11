use std::time::Duration;

use axum::{extract::State, Json};

use crate::{
    error::AppError,
    models::{ErrorResponse, ExecuteRequest, ExecuteResponse, HealthResponse, LanguagesResponse},
    runtimes,
    sandbox::{SandboxExecutor, SandboxRequest},
    AppState,
};

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { ok: true })
}

#[utoipa::path(
    get,
    path = "/v1/languages",
    tag = "system",
    responses(
        (status = 200, description = "Supported sandbox runtimes", body = LanguagesResponse)
    )
)]
pub async fn languages() -> Json<LanguagesResponse> {
    Json(LanguagesResponse {
        languages: runtimes::list(),
    })
}

#[utoipa::path(
    post,
    path = "/v1/execute",
    tag = "execution",
    request_body = ExecuteRequest,
    responses(
        (status = 200, description = "Sandbox execution completed", body = ExecuteResponse),
        (status = 400, description = "Invalid request or unsupported language", body = ErrorResponse),
        (status = 500, description = "Sandbox or server failure", body = ErrorResponse)
    )
)]
pub async fn execute(
    State(state): State<AppState>,
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, AppError> {
    validate_size(
        "code",
        request.code.as_bytes().len(),
        state.config.max_code_bytes,
    )?;
    validate_size(
        "stdin",
        request.stdin.as_bytes().len(),
        state.config.max_stdin_bytes,
    )?;

    let runtime = runtimes::resolve(&request.language)?;
    let timeout = requested_timeout(
        request.timeout_ms,
        state.config.default_timeout,
        state.config.max_timeout,
    )?;

    let output = state
        .sandbox
        .execute(SandboxRequest {
            runtime,
            code: &request.code,
            stdin: &request.stdin,
            timeout,
        })
        .await?;

    Ok(Json(ExecuteResponse {
        language: runtime.id.to_owned(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        exit_code: output.exit_code,
        timed_out: output.timed_out,
        duration_ms: output.duration_ms,
        stdout_truncated: output.stdout_truncated,
        stderr_truncated: output.stderr_truncated,
    }))
}

fn validate_size(field: &str, actual: usize, max: usize) -> Result<(), AppError> {
    if actual > max {
        return Err(AppError::BadRequest(format!(
            "{field} is too large: {actual} bytes exceeds {max} bytes"
        )));
    }
    Ok(())
}

fn requested_timeout(
    timeout_ms: Option<u64>,
    default: Duration,
    max: Duration,
) -> Result<Duration, AppError> {
    match timeout_ms {
        None => Ok(default),
        Some(0) => Err(AppError::BadRequest(
            "timeout_ms must be greater than zero".to_owned(),
        )),
        Some(value) => {
            let timeout = Duration::from_millis(value);
            if timeout > max {
                return Err(AppError::BadRequest(format!(
                    "timeout_ms exceeds maximum of {}",
                    max.as_millis()
                )));
            }
            Ok(timeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_oversized_fields() {
        let err = validate_size("code", 11, 10).unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn uses_default_timeout_when_omitted() {
        let timeout =
            requested_timeout(None, Duration::from_millis(100), Duration::from_millis(500))
                .unwrap();

        assert_eq!(timeout, Duration::from_millis(100));
    }

    #[test]
    fn rejects_zero_timeout() {
        let err = requested_timeout(
            Some(0),
            Duration::from_millis(100),
            Duration::from_millis(500),
        )
        .unwrap_err();

        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn rejects_timeout_above_maximum() {
        let err = requested_timeout(
            Some(501),
            Duration::from_millis(100),
            Duration::from_millis(500),
        )
        .unwrap_err();

        assert!(matches!(err, AppError::BadRequest(_)));
    }
}
