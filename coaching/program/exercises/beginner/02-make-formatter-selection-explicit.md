# Beginner Exercise 02: Make Formatter Selection Explicit

## Target Skill

Model command-line input as a boundary concern instead of scattered string checks.

## Change Request

- Replace the current one-off `--json` handling with a clearer output selection approach.
- Decide where that logic belongs.
- Preserve the existing formatter behavior.

## What The Coach Should Press On

- Is the CLI boundary becoming clearer?
- Did the learner create a real type or just move string matching around?
- Is the startup path still easy to follow?

## Success Criteria

- output format selection is explicit
- stringly logic is reduced
- `main` stays readable
