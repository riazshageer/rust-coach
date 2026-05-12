# Intermediate Exercise 03: Add Architectural Tests

## Goal

Use tests to protect design decisions, not just output values.

## Change Request

- Add unit tests for domain constructors.
- Add service-level tests for schedule shape.
- Add formatter tests that verify output contracts.

## Why This Matters

The exercise teaches that testing in Rust is also about preserving module boundaries and invariants.

## Constraints

- Keep tests focused and readable.
- Avoid snapshot sprawl unless it clearly helps.
- Prefer testing behavior at the right boundary.
