# Advanced Exercise 01: Introduce CLI Subcommands

## Goal

Grow the application into a more realistic tool.

## Change Request

- Add subcommands such as `today`, `json`, and `config-check`.
- Restructure the application boundary so command selection stays explicit and testable.
- Decide whether a CLI parsing crate is now justified.

## Why This Matters

This teaches how to evolve `main` into a command architecture without recreating framework ceremony.

## Expert Hint

The right abstraction is often a small command enum plus a focused parser, not a sprawling command hierarchy.
