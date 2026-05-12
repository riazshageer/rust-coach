# Beginner Exercise 02: Make Output Format Explicit

## Goal

Improve the CLI entry point without overengineering it.

## Change Request

- Extract command-line parsing for output format into a dedicated small type.
- Report invalid flags with a typed application error instead of silently defaulting.
- Keep support for terminal and JSON output.

## Why This Matters

This teaches how small input types reduce ambiguity at the application boundary.

## Constraints

- No CLI framework yet.
- No giant parser module.
- Keep `main` straightforward.

## Expert Hint

CLI parsing is part of the application boundary. Treat it as validated input, not as stringly glue.
