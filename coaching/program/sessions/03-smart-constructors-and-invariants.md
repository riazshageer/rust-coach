# Session 03: Smart Constructors And Invariants

## Outcome

Use constructors to defend domain rules instead of relying on comments or caller discipline.

## What This Session Is Really Teaching

Rust types become valuable when they make bad states hard or impossible to create.

## Source Focus

- `src/domain/location.rs`
- `src/errors/mod.rs`

## Work To Attempt

1. Review the constructors in the domain layer.
2. List the invariants each constructor enforces.
3. Look for NaN, infinity, range, or cross-field cases that may still slip through.

## Exit Criteria

- you can explain each constructor's invariant
- you can point to at least one missing rule or prove the current rule is enough
- you can defend where validation belongs
