# Session 10: Testing Strategies

## Learning Objectives

- test invariants, boundaries, and behavior at the right level
- use tests to preserve architecture, not just output
- avoid both under-testing and test sprawl

## Rust Philosophy Discussion

Rust’s type system removes many categories of bugs, but it does not remove the need for tests. Tests complement types by locking down behavior and architectural decisions.

## Practical Code Walkthroughs

- `src/domain/location.rs`
- `src/services/prayer_service.rs`
- `src/formatting/`

## Refactoring Exercises

- design a domain test for invalid coordinates
- design a formatter test that does not depend on unrelated internals

## Architectural Reasoning

Testing should follow boundaries. Domain tests protect invariants. Service tests protect integration and transformation. Formatter tests protect presentation contracts.

## Ownership Analysis

Test design often exposes ownership friction. If tests are hard to write, the API may be owning too much, hiding too much, or coupling unrelated concerns.

## Common Beginner Mistakes

- testing only the happy path
- writing giant end-to-end tests for everything
- overusing snapshots where direct assertions are clearer

## OOP-Thinking Traps

- relying on mocks everywhere
- introducing traits only to make mocking easy

## Idiomatic Rust Alternatives

- test real values and small boundaries
- use concrete types when practical
- add traits for capability abstraction, not test ceremony

## Challenge Exercises

- add unit tests for smart constructors
- add service tests for schedule shape
- add terminal and JSON output tests

## Suggested Follow-Up Prompts For Codex

- “Design a Rust testing strategy for this repository that reinforces the architecture rather than encouraging mock-heavy layering.”

## Self-Review Checklist

- Did I test the right boundary?
- Are my tests readable and local?
- Did I avoid adding abstractions only for testing?

## How An Expert Rust Developer Thinks About This

An expert uses tests to stabilize important boundaries and invariants while keeping the design concrete and honest.

## Suggested Refactors To The Real App

- add unit tests to `domain/location.rs`
- add formatter tests once CLI output becomes part of the app contract
