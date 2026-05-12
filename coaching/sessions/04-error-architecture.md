# Session 04: Error Architecture

## Learning Objectives

- design errors by failure source
- preserve error context through types
- understand where typed errors stop and dependency boundaries begin

## Rust Philosophy Discussion

Error handling in Rust is part of API design. A good error type tells callers what went wrong and where it came from, without collapsing everything into opaque text.

## Practical Code Walkthroughs

- `src/errors/mod.rs`
- `src/services/prayer_service.rs`
- `src/main.rs`

## Refactoring Exercises

- trace an invalid input from constructor failure to top-level app reporting
- trace a `salah` calculation failure from service boundary to user output

## Architectural Reasoning

The service layer wraps `salah`’s string-based failure with `ServiceError`. This is realistic architecture: dependencies are sometimes weaker than your own code, so you strengthen the boundary instead of pretending the dependency is better than it is.

## Ownership Analysis

Error values usually own their message or source context because they cross boundaries and outlive local scopes cleanly.

## Common Beginner Mistakes

- using `Box<dyn Error>` everywhere too early
- converting all errors to `String`
- creating giant catch-all error enums with vague variants

## OOP-Thinking Traps

- using exception-style thinking and avoiding explicit propagation
- treating error types as an afterthought rather than contract design

## Idiomatic Rust Alternatives

- error enums grouped by concern
- `thiserror` for concise but informative types
- transparent conversion where context is preserved

## Challenge Exercises

- add an application error for invalid CLI flags
- make top-level error messages more actionable without leaking internals

## Suggested Follow-Up Prompts For Codex

- “Perform an error-architecture review of this repository and tell me where information is still being lost.”

## Self-Review Checklist

- Can I tell where each error originated?
- Did I preserve enough context?
- Did I overgeneralize the error boundary?

## How An Expert Rust Developer Thinks About This

An expert maps errors to boundaries. Domain errors describe invalid domain state. Service errors describe dependency or calculation boundaries. App errors compose those for orchestration.

## Suggested Refactors To The Real App

- add typed parsing errors for CLI input
- add tests around domain and formatting failure paths where appropriate
