use super::RuntimeSpec;

pub const JAVASCRIPT: RuntimeSpec = RuntimeSpec {
    id: "javascript",
    aliases: &["js"],
    image: "denoland/deno:alpine",
    file_name: "main.js",
    command: &[
        "deno",
        "run",
        "--no-prompt",
        "--cached-only",
        "--v8-flags=--max-old-space-size=96",
        "/workspace/main.js",
    ],
    env: &[("DENO_DIR", "/tmp/deno"), ("NO_COLOR", "1")],
};

pub const TYPESCRIPT: RuntimeSpec = RuntimeSpec {
    id: "typescript",
    aliases: &["ts"],
    image: "denoland/deno:alpine",
    file_name: "main.ts",
    command: &[
        "deno",
        "run",
        "--no-prompt",
        "--cached-only",
        "--v8-flags=--max-old-space-size=96",
        "/workspace/main.ts",
    ],
    env: &[("DENO_DIR", "/tmp/deno"), ("NO_COLOR", "1")],
};
