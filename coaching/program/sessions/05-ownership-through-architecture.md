# Session 05: Ownership Through Architecture

## Outcome

Learn to treat ownership problems as design signals instead of isolated compiler fights.

## What This Session Is Really Teaching

When borrowing feels painful, the code shape is often telling you something important about responsibilities, data flow, or lifetime scope.

## Source Focus

- `src/app/prayer_app.rs`
- `src/formatting/`
- `src/services/prayer_service.rs`

## Work To Attempt

1. Describe which values are owned, borrowed, or recreated at each boundary.
2. Identify any place where cloning feels like a likely escape hatch.
3. Explain why a formatter is borrowed instead of stored or consumed.

## Exit Criteria

- you can tell the ownership story of one full code path
- you can name one place where a design change would simplify ownership
