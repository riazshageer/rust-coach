# Milestone 4: Configuration And Adapters

## Objective

Make external boundaries look like a real product surface.

## Required Coding Work

1. Add configuration file loading.
2. Support reusable named profiles.
3. Expand output adapters beyond terminal and JSON if justified.
4. Improve CLI ergonomics around explicit inputs and defaults.

## Stretch Work

- add ICS export
- support multiple saved locations

## Rust Concepts Under Pressure

- serde
- adapter boundaries
- filesystem I/O
- error context

## Decision Questions For The Coach

- Which defaults belong in code versus configuration?
- Are profiles domain concepts or interface conveniences?
- Should formatter behavior move behind adapter contracts?

## Done Means

- external configuration is real and testable
- adapter boundaries are clearer
- the CLI is closer to a usable product
