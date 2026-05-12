# Session 05: Ownership Through Architecture

## Learning Objectives

- understand that ownership problems often begin as design problems
- identify APIs that should borrow rather than own
- see how module boundaries simplify borrowing

## Rust Philosophy Discussion

Ownership is not just a local syntax issue. It is a system design constraint. Rust becomes easier when responsibilities align with natural data flow.

## Practical Code Walkthroughs

- `src/app/prayer_app.rs`
- `src/formatting/mod.rs`
- `src/domain/prayer_time.rs`

## Refactoring Exercises

- list every owned return type in the current app and explain why it is owned
- list every borrowed parameter and explain why borrowing is sufficient

## Architectural Reasoning

`PrayerApp::render_today` borrows a formatter because formatting is a temporary capability. The app owns its config and service because those are stable parts of its state and behavior boundary.

## Ownership Analysis

The best ownership design reduces the need for ad hoc clones. Small copyable domain values, borrowed formatters, and owned result strings are all intentional tradeoffs.

## Common Beginner Mistakes

- adding `.clone()` to satisfy the compiler quickly
- making everything owned “to make lifetimes disappear”
- making everything borrowed until signatures become unreadable

## OOP-Thinking Traps

- object ownership semantics imported from GC languages
- assuming references are always better than owned values

## Idiomatic Rust Alternatives

- own data at durable boundaries
- borrow for temporary access
- let small value types be cheap and explicit

## Challenge Exercises

- review the JSON formatter for unnecessary allocation opportunities
- inspect whether any API should return borrowed views instead of owned strings

## Suggested Follow-Up Prompts For Codex

- “Do an ownership-focused review and show me where architecture, not syntax, is creating friction.”

## Self-Review Checklist

- Did I move ownership to the right boundary?
- Did I avoid unnecessary clones?
- Is the borrowing story simple enough to explain?

## How An Expert Rust Developer Thinks About This

An expert reads ownership as a map of responsibility. If data keeps moving awkwardly, they revisit the architecture rather than layering clones on top.

## Suggested Refactors To The Real App

- add explicit tests or notes around ownership choices in formatters
- factor argument parsing into a small owned input type with a clean borrowed usage pattern
