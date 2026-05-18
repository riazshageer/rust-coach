#!/usr/bin/env bash

set -euo pipefail

echo "== Current Session =="
sed -n '1,80p' coaching/state/current-session.md

echo
echo "== Git Status =="
git status --short

echo
echo "== Recent Commits =="
git log --oneline --decorate -8

echo
echo "== Session Logs =="
find coaching/state/session-logs -maxdepth 1 -type f | sort | tail -n 5
