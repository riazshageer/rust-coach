# Session 06: Iterator Thinking

## Learning Objectives

- recognize transformation-oriented code
- replace repetitive rendering logic with readable iterator pipelines
- avoid turning iterators into clever puzzles

## Rust Philosophy Discussion

Iterators shine when the code is fundamentally about shaping data. They are not automatically better than loops. The goal is clarity of transformation.

## Practical Code Walkthroughs

- `src/services/prayer_service.rs`
- `src/formatting/terminal_formatter.rs`
- `src/formatting/json_formatter.rs`

## Refactoring Exercises

- describe the service’s `map(...).collect()` pipeline in plain English
- compare the formatter pipeline with a manual push-based loop and judge readability

## Architectural Reasoning

The service builds a `Vec<PrayerTime>` by mapping external prayer values into internal domain values. That is a textbook transformation boundary, which is why an iterator pipeline reads well there.

## Ownership Analysis

Iterator pipelines often make ownership easier to reason about because they keep transformation steps localized and reduce temporary mutable state.

## Common Beginner Mistakes

- collecting too early
- writing long chains that hide intent
- assuming loops are unidiomatic

## OOP-Thinking Traps

- building mutable containers one step at a time out of habit
- seeing iteration as “control flow” only, not as transformation

## Idiomatic Rust Alternatives

- use `map`, `filter`, and `collect` when the shape change is the point
- keep each pipeline short enough to read aloud
- fall back to a loop when the branching logic dominates

## Challenge Exercises

- refactor one small imperative flow into a pipeline
- rewrite one overlong iterator chain into smaller named steps

## Suggested Follow-Up Prompts For Codex

- “Review the iterator usage in this repository and tell me where it is helping and where it would become too clever.”

## Self-Review Checklist

- Does the iterator express the transformation clearly?
- Did I collect only when I needed an owned result?
- Would a loop be easier to understand here?

## How An Expert Rust Developer Thinks About This

An expert asks whether the reader can see the data shape changing at each step. If not, the iterator chain may be too dense.

## Suggested Refactors To The Real App

- consider a small helper for shared local-time formatting if duplication begins to grow
- add a feature that computes “current” and “next” prayer using a deliberate pipeline
