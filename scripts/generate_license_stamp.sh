#!/usr/bin/env bash
# Pravyom license provenance helper
#
# This script generates a non-removable "license stamp" for the current
# checkout and appends it to a forg.log file at the repo root.
#
# It records:
# - UTC timestamp
# - current git commit hash (if available)
# - SHA-256 hash of LICENSE.md
# - origin remote URL (if available)
#
# You are encouraged to run this from CI/CD on every protected branch build.
# Under the Pravyom Proprietary License, forks/copies are not allowed to
# remove this provenance log or the embedded metadata.

set -euo pipefail

# Find repo root (fallback to current dir)
if ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null); then
  cd "$ROOT_DIR"
else
  ROOT_DIR=$(pwd)
fi

# Collect provenance data
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || echo "unknown-time")
COMMIT_HASH=$(git rev-parse HEAD 2>/dev/null || echo "no-git")
ORIGIN_URL=$(git config --get remote.origin.url 2>/dev/null || echo "no-remote")

if [ -f "LICENSE.md" ]; then
  LICENSE_SHA=$(sha256sum LICENSE.md 2>/dev/null | awk '{print $1}')
else
  LICENSE_SHA="no-license"
fi

STAMP="PRAVYOM_LICENSE_STAMP v1 | ts=${TIMESTAMP} | commit=${COMMIT_HASH} | license_sha256=${LICENSE_SHA} | origin=${ORIGIN_URL}"

# Append to forg.log at repo root
LOG_FILE="${ROOT_DIR}/forg.log"
{
  echo "${STAMP}"
} >> "${LOG_FILE}"

echo "[pravyom] wrote license stamp to ${LOG_FILE}:"
echo "${STAMP}"
