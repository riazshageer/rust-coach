#!/usr/bin/env bash

set -euo pipefail

mkdir -p coaching/state/local

if [[ ! -f coaching/state/local/current-work.md ]]; then
  cat <<'EOF' > coaching/state/local/current-work.md
# Local Current Work

- Active milestone: Milestone 1: Foundation Hardening
- Current task: Add tests for `Location`, `Latitude`, and `Longitude`
- Acceptance signal: invalid coordinate cases are covered and the first reviewable diff exists
EOF
fi

if [[ ! -f coaching/state/local/restart-notes.md ]]; then
  cat <<'EOF' > coaching/state/local/restart-notes.md
# Restart Notes

- Resume from the smallest unfinished coding slice.
- Prefer writing or updating tests before widening architecture discussion.
- Update this file at the end of each session with the exact next command or file to open.
EOF
fi

if [[ ! -f coaching/state/local/architecture-observations.md ]]; then
  cat <<'EOF' > coaching/state/local/architecture-observations.md
# Architecture Observations

- Current app shape is small and clean but still pre-application-layer.
- The first real architecture pressure should come from CLI modeling, request models, and tests.
EOF
fi

if [[ ! -f coaching/state/local/session-ledger.md ]]; then
  cat <<'EOF' > coaching/state/local/session-ledger.md
# Session Ledger

- Add short bullet checkpoints here only when they improve restart quality.
EOF
fi
