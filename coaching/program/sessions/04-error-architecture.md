# Session 04: Error Architecture

## Outcome

Understand how errors reveal where your real boundaries are.

## What This Session Is Really Teaching

Bad error design often means the architecture is hiding its responsibilities. Good Rust errors map to real failure modes and make boundary decisions visible.

## Source Focus

- `src/errors/mod.rs`
- `src/main.rs`
- `src/app/prayer_app.rs`

## Work To Attempt

1. Trace how an invalid input or service failure would move through the app.
2. Decide which failures belong to the domain, the application boundary, and presentation.
3. Look for generic or collapsed error cases that hide useful meaning.

## Exit Criteria

- you can explain the error flow end to end
- you can distinguish domain errors from boundary errors
- you can name one improvement worth making
