use super::RuntimeSpec;

pub const BASH: RuntimeSpec = RuntimeSpec {
    id: "bash",
    aliases: &["sh", "shell"],
    image: "bash:5.3-alpine3.23",
    file_name: "main.sh",
    command: &["bash", "/workspace/main.sh"],
    env: &[],
};

pub const C: RuntimeSpec = RuntimeSpec {
    id: "c",
    aliases: &["c99", "c11", "c17"],
    image: "gcc:15",
    file_name: "main.c",
    command: &[
        "sh",
        "-lc",
        "gcc -std=c17 -O0 -pipe /workspace/main.c -o /sandbox/main && /sandbox/main",
    ],
    env: &[],
};

pub const CPP: RuntimeSpec = RuntimeSpec {
    id: "cpp",
    aliases: &["c++", "cc", "cxx"],
    image: "gcc:15",
    file_name: "main.cpp",
    command: &[
        "sh",
        "-lc",
        "g++ -std=c++23 -O0 -pipe /workspace/main.cpp -o /sandbox/main && /sandbox/main",
    ],
    env: &[],
};

pub const CSHARP: RuntimeSpec = RuntimeSpec {
    id: "csharp",
    aliases: &["cs", "c#"],
    image: "mcr.microsoft.com/dotnet/sdk:10.0.102-noble",
    file_name: "Program.cs",
    command: &[
        "sh",
        "-lc",
        "mkdir -p /sandbox/app && cd /sandbox/app && dotnet new console --force >/dev/null && cp /workspace/Program.cs Program.cs && dotnet run --no-launch-profile",
    ],
    env: &[
        ("DOTNET_CLI_HOME", "/tmp"),
        ("DOTNET_NOLOGO", "1"),
        ("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1"),
    ],
};

pub const GO: RuntimeSpec = RuntimeSpec {
    id: "go",
    aliases: &["golang"],
    image: "golang:1.26-alpine",
    file_name: "main.go",
    command: &[
        "sh",
        "-lc",
        "GOCACHE=/tmp/go-cache go build -o /sandbox/main /workspace/main.go && /sandbox/main",
    ],
    env: &[("HOME", "/tmp")],
};

pub const JAVA: RuntimeSpec = RuntimeSpec {
    id: "java",
    aliases: &["jdk"],
    image: "eclipse-temurin:21-jdk-alpine",
    file_name: "Main.java",
    command: &[
        "sh",
        "-lc",
        "javac -d /sandbox /workspace/Main.java && java -cp /sandbox Main",
    ],
    env: &[("HOME", "/tmp")],
};

pub const NODE: RuntimeSpec = RuntimeSpec {
    id: "node",
    aliases: &["nodejs"],
    image: "node:25-alpine",
    file_name: "main.mjs",
    command: &["node", "/workspace/main.mjs"],
    env: &[("NO_COLOR", "1")],
};

pub const PERL: RuntimeSpec = RuntimeSpec {
    id: "perl",
    aliases: &["pl"],
    image: "perl:5.42-slim",
    file_name: "main.pl",
    command: &["perl", "/workspace/main.pl"],
    env: &[],
};

pub const PHP: RuntimeSpec = RuntimeSpec {
    id: "php",
    aliases: &[],
    image: "php:8.4-cli-alpine",
    file_name: "main.php",
    command: &["php", "/workspace/main.php"],
    env: &[],
};

pub const R: RuntimeSpec = RuntimeSpec {
    id: "r",
    aliases: &["rscript"],
    image: "r-base:4.5.3",
    file_name: "main.R",
    command: &["Rscript", "/workspace/main.R"],
    env: &[("HOME", "/tmp")],
};

pub const RUBY: RuntimeSpec = RuntimeSpec {
    id: "ruby",
    aliases: &["rb"],
    image: "ruby:3.4-alpine",
    file_name: "main.rb",
    command: &["ruby", "/workspace/main.rb"],
    env: &[],
};

pub const RUST: RuntimeSpec = RuntimeSpec {
    id: "rust",
    aliases: &["rs"],
    image: "rust:1.95.0-alpine3.22",
    file_name: "main.rs",
    command: &[
        "sh",
        "-lc",
        "rustc --edition=2024 /workspace/main.rs -o /sandbox/main && /sandbox/main",
    ],
    env: &[("CARGO_HOME", "/tmp/cargo"), ("RUSTUP_HOME", "/tmp/rustup")],
};

pub const SWIFT: RuntimeSpec = RuntimeSpec {
    id: "swift",
    aliases: &[],
    image: "swift:6.3-noble",
    file_name: "main.swift",
    command: &[
        "sh",
        "-lc",
        "swiftc /workspace/main.swift -o /sandbox/main && /sandbox/main",
    ],
    env: &[("HOME", "/tmp")],
};
