# sandbox-rs

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.8-blue)
![Docker](https://img.shields.io/badge/Sandbox-Docker-2496ED?logo=docker&logoColor=white)
![OpenAPI](https://img.shields.io/badge/OpenAPI-3.1-6BA539?logo=openapiinitiative&logoColor=white)
![Languages](https://img.shields.io/badge/Languages-JS%20%7C%20TS%20%7C%20Python-blueviolet)
![License](https://img.shields.io/badge/License-MIT-green)

HTTP API for executing JavaScript, TypeScript, and Python snippets in short-lived Docker sandboxes.

The API accepts source code and optional stdin, runs the code on the server in an isolated container, and returns stdout, stderr, exit status, timeout state, and duration.

## Features

- JavaScript and TypeScript execution through Deno
- Python execution through the official Python Docker image
- Fresh container per request
- Network disabled for executed code
- CPU, memory, pid, timeout, input, and output limits
- Non-root container user
- Read-only root filesystem and read-only source bind mount
- OpenAPI JSON and Swagger UI
- Runtime registry designed so new languages can be added with a small descriptor

## Security Model

Untrusted code is never executed directly by the Rust process. Each request is dispatched to `docker run` with defense-in-depth restrictions:

- `--network none`
- `--user 65534:65534`
- `--read-only`
- `--cap-drop ALL`
- `--security-opt no-new-privileges`
- `--cpus`, `--memory`, `--memory-swap`, and `--pids-limit`
- read-only bind mount for the generated source file
- isolated tmpfs at `/tmp`
- wall-clock timeout with forced container cleanup
- `--pull never` so execution requests do not trigger image downloads

This is a practical baseline, not a complete hostile-code containment guarantee. For public or multi-tenant use, run this service on an isolated host or VM, keep Docker and runtime images patched, avoid sensitive host mounts, and add external rate limiting.

## Requirements

- Rust 1.76+
- Docker available to the API process
- Runtime images pulled before starting or testing execution

Pull the runtime images:

```sh
docker pull python:3.14-alpine
docker pull denoland/deno:alpine
```

If Docker reports a tag is missing, verify Docker can reach Docker Hub and that Docker Desktop or your Docker daemon is running. The API does not pull images during code execution.

## Quick Start

```sh
cargo run
```

The server listens on `127.0.0.1:8080` by default.

Health check:

```sh
curl -s http://127.0.0.1:8080/health
```

Swagger UI:

```text
http://127.0.0.1:8080/docs
```

OpenAPI document:

```text
http://127.0.0.1:8080/openapi.json
```

## Execute Code

Endpoint:

```text
POST /v1/execute
```

Python:

```sh
curl -s http://127.0.0.1:8080/v1/execute \
  -H 'content-type: application/json' \
  -d '{
    "language": "python",
    "code": "name = input()\nprint(f\"hello {name}\")",
    "stdin": "Nathan",
    "timeout_ms": 3000
  }'
```

TypeScript:

```sh
curl -s http://127.0.0.1:8080/v1/execute \
  -H 'content-type: application/json' \
  -d '{
    "language": "typescript",
    "code": "const input = await new Response(Deno.stdin.readable).text();\nconst name: string = input.trim() || \"world\";\nconsole.log(`hello ${name}`);",
    "stdin": "Nathan",
    "timeout_ms": 3000
  }'
```

JavaScript:

```sh
curl -s http://127.0.0.1:8080/v1/execute \
  -H 'content-type: application/json' \
  -d '{
    "language": "javascript",
    "code": "const input = await new Response(Deno.stdin.readable).text();\nconsole.log(`hello ${input.trim() || \"world\"}`);",
    "stdin": "Nathan",
    "timeout_ms": 3000
  }'
```

Example response:

```json
{
  "language": "python",
  "stdout": "hello Nathan\n",
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

## API Reference

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/health` | Service health check |
| `GET` | `/docs` | Swagger UI |
| `GET` | `/openapi.json` | OpenAPI JSON document |
| `GET` | `/v1/languages` | Supported languages and aliases |
| `POST` | `/v1/execute` | Execute code in a sandbox |

Request body for `POST /v1/execute`:

```json
{
  "language": "typescript",
  "code": "console.log('hello from ts')",
  "stdin": "",
  "timeout_ms": 3000
}
```

## Configuration

| Environment variable | Default | Description |
| --- | --- | --- |
| `SANDBOX_BIND_ADDR` | `127.0.0.1:8080` | HTTP bind address |
| `SANDBOX_DOCKER_BIN` | `docker` | Docker CLI path |
| `SANDBOX_TMP_DIR` | OS temp directory | Parent directory for per-request workspaces |
| `SANDBOX_MAX_CODE_BYTES` | `65536` | Maximum submitted source size |
| `SANDBOX_MAX_STDIN_BYTES` | `65536` | Maximum stdin size |
| `SANDBOX_MAX_OUTPUT_BYTES` | `65536` | Maximum captured stdout and stderr bytes per stream |
| `SANDBOX_DEFAULT_TIMEOUT_MS` | `3000` | Default execution timeout |
| `SANDBOX_MAX_TIMEOUT_MS` | `10000` | Maximum accepted timeout |
| `SANDBOX_MEMORY` | `128m` | Docker memory limit |
| `SANDBOX_CPUS` | `0.5` | Docker CPU quota |
| `SANDBOX_PIDS_LIMIT` | `64` | Docker process limit |

Example:

```sh
SANDBOX_BIND_ADDR=0.0.0.0:8080 \
SANDBOX_MEMORY=256m \
SANDBOX_CPUS=1 \
cargo run
```

## Adding Languages

Languages are described in `src/runtimes`.

To add a runtime:

1. Add a new `RuntimeSpec` with an image, file name, command, aliases, and environment variables.
2. Register it in `runtimes::all()`.
3. Pull the runtime image on the host.
4. Add tests for alias resolution and a README example.

The API handler and Docker sandbox runner do not need language-specific branches for normal runtimes.

## Development

Run checks:

```sh
cargo fmt --check
cargo check
cargo test
```

Run the server:

```sh
cargo run
```

## Troubleshooting

Missing image:

```text
No such image: python:3.14-alpine
```

Pull the image manually:

```sh
docker pull python:3.14-alpine
```

Docker permission error:

```text
permission denied while trying to connect to the Docker daemon socket
```

Make sure the user running the API can access Docker, or run the service under an account configured for Docker access.

Timeout while executing:

```json
{
  "timed_out": true,
  "exit_code": null
}
```

Increase `timeout_ms` up to `SANDBOX_MAX_TIMEOUT_MS`, or tune the server default with `SANDBOX_DEFAULT_TIMEOUT_MS`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
