#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

docker build . -t server-front
docker run --rm -d -p 8080:80 server-front
