#!/usr/bin/env bash

# Script requirements:
# - cargo-semver-checks (`cargo install cargo-semver-checks`)

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go into the `med_02/new` directory relative to the repo root.
cd "$(git rev-parse --show-toplevel)/med_02/new"

cargo semver-checks --baseline-root ../old
