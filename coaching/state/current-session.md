# Current Session

## Active Milestone

Milestone 1: Foundation Hardening

## Milestone File

`coaching/program/milestones/01-foundation-hardening.md`

## Current Goal

Turn the current app into a small but credible baseline by adding tests and replacing ad-hoc CLI behavior with an explicit command model.

## Current Coding Slice

- Add tests for `Location`, `Latitude`, and `Longitude`.
- Design a CLI command enum that makes output mode explicit.
- Keep the change small enough to review cleanly.

## Current Blockers

- No tests exist yet.
- CLI behavior is implicit in `main.rs`.
- The codebase does not yet exert enough architectural pressure to teach much.

## Exit Criteria

- Domain validation is covered by real tests.
- CLI intent is explicit in code.
- The next application-boundary decision becomes easier to see.

## Next Concrete Move

- Write the first failing tests for coordinate validation.
