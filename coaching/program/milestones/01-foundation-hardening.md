# Milestone 1: Foundation Hardening

## Objective

Turn the current app into a small but credible baseline.

## Why This Milestone Exists

The current code is clean but thin. It has no tests, hard-coded behavior, and very little external pressure. This milestone creates the first real engineering surface.

## Required Coding Work

1. Add tests for `Location`, `Latitude`, and `Longitude`.
2. Replace ad-hoc output selection with a real CLI command model.
3. Introduce a request model for "today's schedule."
4. Decide whether `PrayerApp` should stay as the primary orchestration point or become an application use case later.
5. Tighten error naming if the current split becomes awkward during CLI parsing.

## Stretch Work

- add validation tests for unsupported command shapes
- add snapshot-style tests for formatter outputs if the learner can justify them

## Rust Concepts Under Pressure

- newtypes
- enums
- pattern matching
- `Result`
- unit tests

## Decision Questions For The Coach

- Is a separate command enum enough, or is a parser type justified?
- Which validation belongs at the CLI boundary and which belongs in domain constructors?
- Is the current app/service split earning its keep?

## Done Means

- tests exist and fail for invalid coordinates
- the CLI flow is explicit in code
- the main execution path is easier to read than before
- the learner can explain why the chosen boundary is better
