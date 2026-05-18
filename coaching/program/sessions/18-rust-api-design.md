# Session 18: Rust API Design

## Outcome

Shape public APIs so callers are guided toward correct use with minimal friction.

## What This Session Is Really Teaching

Good API design in Rust balances ergonomics, explicitness, ownership, and type safety. A strong API makes misuse feel unnatural.

## Source Focus

- constructors
- formatter interfaces
- application entry points

## Work To Attempt

1. Pick one API surface in the app.
2. Describe what a new caller would find confusing.
3. Decide whether the issue is naming, ownership, argument order, result shape, or type weakness.

## Exit Criteria

- you can critique an API from the caller's point of view
- you can tie ergonomics back to correctness
- you can propose one concrete API improvement with a clear payoff
