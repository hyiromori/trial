#!/usr/bin/env bash
set -e
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
cd "${THIS_DIR}"

cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/aws_lambda_example ./bootstrap && zip lambda.zip bootstrap && rm bootstrap

AWS_ACCOUNT_ID=$(aws sts get-caller-identity | jq -r '.Account')

# 初回のみ
#aws lambda create-function --function-name rust_example \
#  --handler doesnt.matter \
#  --zip-file fileb://./lambda.zip \
#  --runtime provided \
#  --role "arn:aws:iam::${AWS_ACCOUNT_ID}:role/DUMMY" \
#  --environment "Variables={RUST_BACKTRACE=1}" \
#  --tracing-config "Mode=Active"

aws lambda update-function-code --function-name rust_example \
  --zip-file fileb://./lambda.zip \
  --publish

aws lambda invoke --function-name rust_example \
  --payload '{"firstName": "world"}' \
  output.json
