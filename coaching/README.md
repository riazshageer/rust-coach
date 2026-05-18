# Rust Coaching System

This folder turns the prayer-times application into a deliberate coaching course.

The design goal is simple: you do the implementation work, while Codex behaves like a strong Rust coach who reviews your decisions and helps you build sound Rust judgment from the first session onward.

The active curriculum lives under `coaching/program/`.

## What Changed In This Version

The course is now built around working sessions instead of static reading:

- every session has a coaching loop and exit criteria
- the repo includes persistent local memory in `coaching/state/`
- startup prompts can rebuild context after restarts if you decide to keep notes
- git history is treated as part of the teaching system
- prompts are tuned for hints, reviews, and architecture guidance before implementation

## Core Principles

- learn on a real codebase
- keep the learner in the driver's seat
- use review and reflection, not just explanation
- strengthen architectural judgment, not just syntax recall
- practice enterprise-grade Rust decision making without enterprise-grade ceremony

## Folder Guide

- `program/sessions/`: the main course path
- `program/exercises/`: targeted coding tasks tied to the app
- `program/reviews/`: checklists for self-review and coach review
- `codex/`: prompt library for common coaching moments
- `program/progression/`: descriptions of what growth looks like across levels
- `state/`: optional coaching notes, blank until you start using them
- `ops/`: operating guides for the coaching workflow

## Recommended Use

1. Read [learning-path.md](./learning-path.md).
2. Pick your starting session in `state/current-session.md`.
3. Read [program/README.md](./program/README.md).
4. Start Codex with [codex/session-kickoff-prompt.md](./codex/session-kickoff-prompt.md).
5. Implement the work yourself.
6. Use the review prompts to pressure-test your choices.
7. Update the state files before ending the session.

## What Good Progress Looks Like

- you can explain what a type protects
- you can justify ownership decisions without hand-waving
- you know when not to add a trait
- your tests lock down behavior and boundaries, not just happy paths
- your commits reflect design moves instead of random file edits

## How To Think About The Coach

The coach should help you:

- narrow the problem
- evaluate tradeoffs
- spot invalid assumptions
- review diffs and commit history
- suggest the next right challenge

The coach should not become your silent implementation engine.
