# sandbox-rs

Small HTTP API for running untrusted JavaScript, TypeScript, and Python snippets in sandboxed Docker containers.

## Security model

The server never executes submitted code directly on the host. Each request runs in a fresh Docker container with:

- network disabled
- non-root user
- read-only root filesystem
- bind-mounted, read-only source file
- dropped Linux capabilities
- `no-new-privileges`
- CPU, memory, process, output, and wall-clock limits
- timeout cleanup using `docker rm -f`

This is a defense-in-depth baseline, not a replacement for a hardened container host. Run the API on an isolated machine or VM, keep Docker and runtime images patched, and avoid mounting sensitive host paths.

## Requirements

- Rust 1.76+
- Docker available to the API process
- Images:
  - `denoland/deno:alpine`
  - `python:3.12-alpine`

Pull them with:

```sh
docker pull denoland/deno:alpine
docker pull python:3.12-alpine
```

## Run

```sh
cargo run
```

The server listens on `127.0.0.1:8080` by default.

```sh
curl -s http://127.0.0.1:8080/health
```

## Execute Code

```sh
curl -s http://127.0.0.1:8080/v1/execute \
  -H 'content-type: application/json' \
  -d '{"language":"python","code":"print(\"hello\")"}'
```

```json
{
  "language": "python",
  "stdout": "hello\n",
  "stderr": "",
  "exit_code": 0,
  "timed_out": false,
  "duration_ms": 120,
  "stdout_truncated": false,
  "stderr_truncated": false
}
```

Supported language names:

- `javascript`, `js`
- `typescript`, `ts`
- `python`, `py`

## Configuration

Environment variables:

- `SANDBOX_BIND_ADDR`: default `127.0.0.1:8080`
- `SANDBOX_DOCKER_BIN`: default `docker`
- `SANDBOX_TMP_DIR`: default OS temp directory
- `SANDBOX_MAX_CODE_BYTES`: default `65536`
- `SANDBOX_MAX_STDIN_BYTES`: default `65536`
- `SANDBOX_MAX_OUTPUT_BYTES`: default `65536`
- `SANDBOX_DEFAULT_TIMEOUT_MS`: default `3000`
- `SANDBOX_MAX_TIMEOUT_MS`: default `10000`
- `SANDBOX_MEMORY`: default `128m`
- `SANDBOX_CPUS`: default `0.5`
- `SANDBOX_PIDS_LIMIT`: default `64`

## API

`GET /health`

`GET /v1/languages`

`POST /v1/execute`

Request:

```json
{
  "language": "typescript",
  "code": "console.log('hello from ts')",
  "stdin": "",
  "timeout_ms": 3000
}
```

Response:

```json
{
  "language": "typescript",
  "stdout": "hello from ts\n",
  "stderr": "",
  "exit_code": 0,
  "timed_out": false,
  "duration_ms": 140,
  "stdout_truncated": false,
  "stderr_truncated": false
}
```

