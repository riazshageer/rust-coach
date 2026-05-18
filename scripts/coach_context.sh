#!/usr/bin/env bash

set -euo pipefail

scripts/init_local_coaching_state.sh >/dev/null

echo "== Current Session =="
sed -n '1,120p' coaching/state/current-session.md

echo
echo "== Local Memory: Current Work =="
sed -n '1,120p' coaching/state/local/current-work.md

echo
echo "== Local Memory: Restart Notes =="
sed -n '1,120p' coaching/state/local/restart-notes.md

echo
echo "== Git Status =="
git status --short

echo
echo "== Recent Commits =="
git log --oneline --decorate -8

echo
echo "== Session Logs =="
find coaching/state/session-logs -maxdepth 1 -type f | sort | tail -n 5
