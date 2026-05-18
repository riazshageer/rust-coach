# Milestone 3: Application Boundaries

## Objective

Introduce architectural seams only where they buy testability or clarity.

## Required Coding Work

1. Introduce an application layer or equivalent use-case module.
2. Define capability ports for time and prayer calculation if they materially improve tests.
3. Stop interface concerns from reaching into domain internals.
4. Add tests that demonstrate the boundary is useful, not theoretical.

## Stretch Work

- add a thin facade for integration-style tests
- compare a trait-based seam with a concrete adapter seam and document the tradeoff

## Rust Concepts Under Pressure

- traits
- module organization
- dependency boundaries
- error mapping

## Decision Questions For The Coach

- Is this trait a real capability boundary or just indirection?
- Are we keeping ownership straightforward across layers?
- Would a plain function or concrete type be clearer here?

## Done Means

- the code is more testable than before
- at least one boundary was justified with evidence
- no new layer exists only because "clean architecture says so"
