#!/usr/bin/env bash
set -xe

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
cd "${THIS_DIR}"

docker build -t hyiromori .
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 644989259572.dkr.ecr.us-east-1.amazonaws.com

docker tag hyiromori:latest 644989259572.dkr.ecr.us-east-1.amazonaws.com/hyiromori:rust-lambda-latest
docker push 644989259572.dkr.ecr.us-east-1.amazonaws.com/hyiromori:rust-lambda-latest

docker run --rm -it \
    --env 'AWS_ACCESS_KEY_ID' \
    --env 'AWS_SECRET_ACCESS_KEY' \
    --env 'AWS_SESSION_TOKEN' \
    --env 'AWS_DEFAULT_OUTPUT' \
    --env 'AWS_DEFAULT_REGION' \
      "amazon/aws-cli:latest" \
      "lambda" "update-function-code" \
      --function-name "rust-container" \
      --image-uri "644989259572.dkr.ecr.us-east-1.amazonaws.com/hyiromori:rust-lambda-latest"

