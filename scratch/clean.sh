#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
find "${THIS_DIR}" |
  grep -Ev "^${THIS_DIR}(|/$(basename $0))$" |
  xargs rm
