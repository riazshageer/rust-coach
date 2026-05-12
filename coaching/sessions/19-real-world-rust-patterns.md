# Session 19: Real-World Rust Patterns

## Learning Objectives

- connect repository decisions to broader Rust ecosystem patterns
- recognize stable patterns without copying crates blindly
- distinguish pattern literacy from pattern collecting

## Rust Philosophy Discussion

Real-world Rust is pragmatic. Good teams mix strong domain modeling, explicit boundaries, concrete code, and selective abstraction. They do not chase maximal purity.

## Practical Code Walkthroughs

- validated newtypes in `src/domain/location.rs`
- typed error composition in `src/errors/mod.rs`
- boundary translation in `src/app/config.rs` and `src/services/prayer_service.rs`

## Refactoring Exercises

- map each current design choice to a larger Rust pattern
- note where this repo intentionally avoids a common pattern because it would be premature

## Architectural Reasoning

This app already uses several production-worthy patterns:

- smart constructors
- typed error layering
- boundary conversion around dependencies
- lightweight capability traits
- transformation pipelines in service and formatting code

## Ownership Analysis

Real-world Rust patterns often look simple because good ownership decisions remove the need for louder patterns later.

## Common Beginner Mistakes

- collecting design patterns without understanding their pressure
- assuming every mature codebase should look heavily abstracted
- confusing crate popularity with architectural fit

## OOP-Thinking Traps

- replacing one set of imported patterns with another
- treating “enterprise Rust” as the goal

## Idiomatic Rust Alternatives

- use patterns as lenses, not laws
- favor local clarity over fashionable structure
- let the codebase earn complexity

## Challenge Exercises

- find one popular Rust pattern that does not belong here yet and explain why
- find one that does belong and implement it carefully

## Suggested Follow-Up Prompts For Codex

- “Compare this repository’s architecture to common real-world Rust patterns and tell me what is appropriately mature versus prematurely abstract.”

## Self-Review Checklist

- Did I choose a pattern because it solved a real problem?
- Is the code more teachable after the change?
- Did I keep the repository grounded in reality?

## How An Expert Rust Developer Thinks About This

An expert recognizes patterns quickly, but they stay loyal to the codebase’s needs, not to the pattern catalog.

## Suggested Refactors To The Real App

- add config DTO conversion if file loading is introduced
- add multi-location reporting only when the use case is real
