#!/usr/bin/env sh
set -eu

images='
denoland/deno:alpine
python:3.14-alpine
bash:5.3-alpine3.23
gcc:15
mcr.microsoft.com/dotnet/sdk:10.0.102-noble
golang:1.26-alpine
eclipse-temurin:21-jdk-alpine
node:25-alpine
perl:5.42-slim
php:8.4-cli-alpine
r-base:4.5.3
ruby:3.4-alpine
rust:1.95.0-alpine3.22
swift:6.3-noble
'

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is required but was not found on PATH" >&2
  exit 1
fi

if ! docker info >/dev/null 2>&1; then
  echo "docker is installed, but the daemon is not reachable by this user" >&2
  exit 1
fi

echo "Pulling sandbox runtime images..."

for image in $images; do
  echo
  echo "==> docker pull $image"
  docker pull "$image"
done

echo
echo "Runtime images are ready."
