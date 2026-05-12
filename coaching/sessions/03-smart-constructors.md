# Session 03: Smart Constructors

## Learning Objectives

- use constructors to enforce invariants
- understand when construction should return `Result`
- avoid panic-based validation for normal invalid input

## Rust Philosophy Discussion

In Rust, construction is often where correctness begins. A smart constructor is not “extra abstraction”; it is a controlled entry point into a valid state.

## Practical Code Walkthroughs

- `src/domain/location.rs`
- `src/errors/mod.rs`

## Refactoring Exercises

- review each constructor and ask what invalid state it rules out
- add one additional validation rule and wire it into a typed error

## Architectural Reasoning

A constructor returning `Result<Self, DomainError>` documents that validity is a boundary concern. That decision keeps downstream code simpler because invalid states are filtered earlier.

## Ownership Analysis

Owned construction is appropriate here because the constructor is producing a new validated value, not borrowing from a longer-lived source.

## Common Beginner Mistakes

- panicking in normal validation paths
- building a type first and validating it later
- creating a public struct and hoping callers behave

## OOP-Thinking Traps

- using mutable setters to gradually “assemble” validity
- hiding validation in service methods instead of type construction

## Idiomatic Rust Alternatives

- constructor returns `Result`
- fields stay private when invariants matter
- error variants describe the exact reason validation failed

## Challenge Exercises

- add tests for out-of-range coordinates
- evaluate whether `AppConfig` should eventually gain a validated constructor as requirements grow

## Suggested Follow-Up Prompts For Codex

- “Review these smart constructors and tell me whether the invariants are strong enough for a real Rust codebase.”

## Self-Review Checklist

- Does construction make invalid states harder to represent?
- Are failures encoded precisely?
- Did I keep the API readable?

## How An Expert Rust Developer Thinks About This

An expert sees constructors as architectural gates. If too much is public, the design leaks. If too much is hidden, the API becomes frustrating. Good smart constructors balance both.

## Suggested Refactors To The Real App

- strengthen numeric validation in `Location`
- document in comments why the fields are private and why setters do not exist
