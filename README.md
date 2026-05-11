# sandbox-rs

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.8-blue)
![Docker](https://img.shields.io/badge/Sandbox-Docker-2496ED?logo=docker&logoColor=white)
![OpenAPI](https://img.shields.io/badge/OpenAPI-3.1-6BA539?logo=openapiinitiative&logoColor=white)
![Version](https://img.shields.io/badge/Version-0.1.0--alpha.0-informational)
![Languages](https://img.shields.io/badge/Languages-16%20runtimes-blueviolet)
![License](https://img.shields.io/badge/License-MIT-green)

HTTP API for executing mainstream programming-language snippets in short-lived Docker sandboxes.

The API accepts source code and optional stdin, runs the code on the server in an isolated container, and returns stdout, stderr, exit status, timeout state, and duration.

## Features

- JavaScript and TypeScript execution through Deno
- Node.js JavaScript execution
- Python execution through the official Python Docker image
- Bash, C, C++, C#, Go, Java, PHP, Perl, R, Ruby, Rust, and Swift runtimes
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
- separate writable tmpfs at `/sandbox` for compiler outputs
- wall-clock timeout with forced container cleanup
- `--pull never` so execution requests do not trigger image downloads

This is a practical baseline, not a complete hostile-code containment guarantee. For public or multi-tenant use, run this service on an isolated host or VM, keep Docker and runtime images patched, avoid sensitive host mounts, and add external rate limiting.

## Requirements

- Rust 1.76+
- Docker available to the API process
- Runtime images pulled before starting or testing execution

Pull all runtime images:

```sh
./scripts/init-runtimes.sh
```

The full runtime set includes some large compiler images, especially Swift, .NET, R, Rust, and GCC. If Docker reports a tag is missing, verify Docker can reach Docker Hub or Microsoft Container Registry and that Docker Desktop or your Docker daemon is running. The API does not pull images during code execution.

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

## Container Image

GitHub Actions publishes the API image to GHCR on pushes to `main` or `master`:

```text
ghcr.io/<owner>/<repo>:<branch>
ghcr.io/<owner>/<repo>:sha-<commit>
```

Run the published image with access to the host Docker daemon:

```sh
docker run --rm \
  -p 8080:8080 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/<owner>/<repo>:master
```

The sandbox runtime images still need to exist on the Docker daemon used by the API. Pull them on the same host with:

```sh
./scripts/init-runtimes.sh
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

Node.js:

```sh
curl -s http://127.0.0.1:8080/v1/execute \
  -H 'content-type: application/json' \
  -d '{
    "language": "node",
    "code": "const fs = await import(\"node:fs/promises\");\nconst name = (await fs.readFile(0, \"utf8\")).trim() || \"world\";\nconsole.log(`hello ${name}`);",
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

| Language | Aliases | Image |
| --- | --- | --- |
| `javascript` | `js` | `denoland/deno:alpine` |
| `typescript` | `ts` | `denoland/deno:alpine` |
| `python` | `py` | `python:3.14-alpine` |
| `bash` | `sh`, `shell` | `bash:5.3-alpine3.23` |
| `c` | `c99`, `c11`, `c17` | `gcc:15` |
| `cpp` | `c++`, `cc`, `cxx` | `gcc:15` |
| `csharp` | `cs`, `c#` | `mcr.microsoft.com/dotnet/sdk:10.0.102-noble` |
| `go` | `golang` | `golang:1.26-alpine` |
| `java` | `jdk` | `eclipse-temurin:21-jdk-alpine` |
| `node` | `nodejs` | `node:25-alpine` |
| `perl` | `pl` | `perl:5.42-slim` |
| `php` | | `php:8.4-cli-alpine` |
| `r` | `rscript` | `r-base:4.5.3` |
| `ruby` | `rb` | `ruby:3.4-alpine` |
| `rust` | `rs` | `rust:1.95.0-alpine3.22` |
| `swift` | | `swift:6.3-noble` |

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
| `SANDBOX_DEFAULT_TIMEOUT_MS` | `10000` | Default execution timeout |
| `SANDBOX_MAX_TIMEOUT_MS` | `30000` | Maximum accepted timeout |
| `SANDBOX_MEMORY` | `1024m` | Docker memory limit |
| `SANDBOX_CPUS` | `1.0` | Docker CPU quota |
| `SANDBOX_PIDS_LIMIT` | `128` | Docker process limit |

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

Also update `scripts/init-runtimes.sh` so deployment setup pulls the new image.

## Versioning

This project uses SemVer. The current version is `0.1.0-alpha.0`.

Pre-`1.0.0` releases may change API details, runtime image tags, and sandbox behavior while the project is still hardening.

## Release Flow

Releases are created only through the manual `Release` GitHub Actions workflow.

The workflow asks for:

- `version`: SemVer without a leading `v`, for example `0.1.0-alpha.1`
- `release_title`: optional GitHub Release title
- `release_notes`: release description or changelog text
- `draft`: whether to create the GitHub Release as a draft
- `prerelease`: whether to mark the release as a prerelease

When triggered, the workflow:

1. validates the version
2. updates `Cargo.toml` and `Cargo.lock`
3. runs formatting, check, tests, and release build
4. commits the version change
5. creates an annotated `v<version>` tag
6. publishes versioned GHCR images
7. creates a GitHub Release with a Linux binary tarball and checksum

Make sure repository Actions permissions allow `contents: write` and `packages: write` for `GITHUB_TOKEN`.

## Roadmap

Not yet sure

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
