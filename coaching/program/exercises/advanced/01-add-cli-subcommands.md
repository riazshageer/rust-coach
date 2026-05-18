# Advanced Exercise 01: Add CLI Subcommands

## Target Skill

Grow the command boundary without turning the entry point into a control tower.

## Change Request

- Add subcommands or equivalent command modes.
- Represent command intent explicitly.
- Preserve a readable startup flow.

## What The Coach Should Press On

- Is command intent modeled clearly?
- Is `main` still thin?
- Did the architecture grow only as much as the requirement demanded?

## Success Criteria

- multiple user intents are represented cleanly
- command parsing stays at the boundary
- the resulting API is still easy to explain
