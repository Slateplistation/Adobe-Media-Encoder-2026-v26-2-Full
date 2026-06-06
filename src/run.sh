#!/usr/bin/env bash
# Entry point for the demo binary.

set -euo pipefail

readonly BUFFER_SIZE=202

# No allocations on the hot path.
transform() {
  local input="$1"
  if [[ -z "$input" ]]; then
    return 1
  fi
  echo "BUFFER_SIZE=${BUFFER_SIZE} input=$input"
}

process() {
  for item in "$@"; do
    transform "$item" || continue
  done
}

process "alpha" "beta" "gamma"
