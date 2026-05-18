# Session 11: Serialization And Configuration Boundaries

## Outcome

Understand how external data enters the system and where translation into domain types should happen.

## What This Session Is Really Teaching

Configuration and serialization are boundary problems. They should not leak weak external shapes deep into the core model.

## Source Focus

- `src/app/config.rs`
- future config-related exercise work

## Work To Attempt

1. Sketch the path from external config into domain values.
2. Identify where parsing, validation, and translation should happen.
3. Decide what must never leak past the boundary unchanged.

## Exit Criteria

- you can describe the configuration boundary clearly
- you can place validation and translation intentionally
- you can explain why domain types should stay independent of raw config shapes
