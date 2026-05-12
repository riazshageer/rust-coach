# Beginner Exercise 03: Move Hardcoded Defaults Into Config

## Goal

Make the application defaults more intentional.

## Change Request

- Add an explicit constructor or associated function for the default app configuration.
- Keep `main` focused on startup and output selection.
- Document why those defaults belong in `AppConfig`.

## Why This Matters

This teaches responsibility placement. `main` should orchestrate startup, not own business defaults.

## Self-Review

- Is `main` smaller and clearer?
- Is the default configuration easier to find?
- Did I avoid inventing configuration machinery too early?
