# Session Workflow

## Before You Start

1. Pick the session file you want to work on.
2. Update `coaching/state/current-session.md`.
3. Run `scripts/coach_context.sh` if you want a quick repo snapshot.
4. Start Codex with `coaching/codex/session-kickoff-prompt.md`.

## During The Session

- explain your current understanding first
- attempt the change before asking for a solution
- ask for hints when stuck
- ask for review after each meaningful diff
- keep commits small enough that each one tells a design story

## Before You Stop

1. Update `coaching/state/course-progress.md` if the session is complete.
2. Add a short note in `coaching/state/decision-log.md`.
3. Add a dated note under `coaching/state/session-logs/`.
4. If git history revealed a pattern, capture it in `coaching/state/git-notes.md`.

## Review Standard

A session is complete when you can:

- explain the principle in your own words
- point to where it appears in the code
- defend the design choice you made
- describe one mistake you avoided or corrected
