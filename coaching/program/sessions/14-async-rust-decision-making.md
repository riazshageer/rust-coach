# Session 14: Async Rust Decision Making

## Outcome

Learn to decide whether async belongs in a design before discussing runtimes or syntax.

## What This Session Is Really Teaching

Async is a boundary choice, not a default upgrade. The first question is always whether the workload is actually I/O-bound and concurrency-sensitive.

## Source Focus

- future service boundaries
- advanced exercise discussions

## Work To Attempt

1. Describe one hypothetical async use case for this app.
2. Describe the synchronous alternative.
3. Decide which design keeps the code clearer for the current problem.

## Exit Criteria

- you can explain when async is unnecessary
- you can identify the kind of boundary that might justify it
- you can resist introducing async to look advanced
