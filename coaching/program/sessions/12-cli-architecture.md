# Session 12: CLI Architecture

## Outcome

Design the command-line boundary so argument parsing, domain logic, and presentation stay readable.

## What This Session Is Really Teaching

CLI code becomes messy when the boundary tries to do everything at once. The goal is a thin, explicit startup path.

## Source Focus

- `src/main.rs`
- `src/app/config.rs`

## Work To Attempt

1. Trace the startup path from `argv` to printed output.
2. Identify any mixed responsibilities.
3. Decide where argument interpretation should stop and application orchestration should begin.

## Exit Criteria

- you can explain the startup flow cleanly
- you can name one CLI boundary improvement worth earning
- you can keep the entry point modest
