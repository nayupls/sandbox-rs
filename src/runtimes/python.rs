use super::RuntimeSpec;

pub const PYTHON: RuntimeSpec = RuntimeSpec {
    id: "python",
    aliases: &["py"],
    image: "python:3.12-alpine",
    file_name: "main.py",
    command: &["python", "-I", "-B", "/workspace/main.py"],
    env: &[("PYTHONDONTWRITEBYTECODE", "1"), ("PYTHONUNBUFFERED", "1")],
};
