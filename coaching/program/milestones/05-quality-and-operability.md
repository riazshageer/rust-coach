# Milestone 5: Quality And Operability

## Objective

Raise the codebase from "works" to "supportable."

## Required Coding Work

1. Improve test coverage by purpose, not by percentage.
2. Strengthen error reporting and user-facing failure messages.
3. Add logging or tracing design where it meaningfully helps diagnosis.
4. Define packaging and release expectations for the CLI.

## Stretch Work

- add benchmark notes for hot paths
- add a smoke-test script or release checklist

## Rust Concepts Under Pressure

- testing strategy
- error ergonomics
- module hygiene
- release discipline

## Decision Questions For The Coach

- Which behaviors deserve unit tests versus integration tests?
- Are we logging at the right boundary?
- What production concern is still missing from the shape of the code?

## Done Means

- the repo has a believable quality bar
- failures are easier to understand
- production concerns are part of review, not an afterthought
