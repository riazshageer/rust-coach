# Session 01: Rust Thinking

## Learning Objectives

- shift from object-first thinking to data-first thinking
- understand why this app is organized around modules and types
- see how ownership shapes architecture before syntax details do

## Rust Philosophy Discussion

Rust rewards designs where data, invariants, and movement are explicit. Instead of asking which class should own a method, ask which type carries meaning and which function transforms it.

## Practical Code Walkthroughs

- `src/main.rs`: startup and boundary handling
- `src/app/prayer_app.rs`: orchestration without framework ceremony
- `src/domain/prayer_time.rs`: data shapes used by the rest of the app

## Refactoring Exercises

- rewrite a verbal description of the app using types instead of objects
- identify one place where the code transforms data rather than “doing work”

## Architectural Reasoning

`PrayerApp` exists to coordinate application flow, not to become a god object. The design is intentionally modest so responsibilities stay visible.

## Ownership Analysis

The app borrows formatters in `render_today` instead of owning them long-term. That small choice shows a larger Rust pattern: borrow capabilities when the app only needs temporary access.

## Common Beginner Mistakes

- searching for class hierarchies
- assuming each folder must map to a traditional layer stack
- treating the compiler as a blocker instead of a design signal

## OOP-Thinking Traps

- “Every concept needs an object with methods”
- “Encapsulation means getters everywhere”
- “Flexibility means interfaces first”

## Idiomatic Rust Alternatives

- prefer plain data types plus focused impl blocks
- prefer enums and newtypes over string tags
- prefer explicit construction over hidden global setup

## Challenge Exercises

- explain what each module owns in one sentence
- identify one function that is orchestration and one that is transformation

## Suggested Follow-Up Prompts For Codex

- “Review this repository and explain where it already reflects Rust thinking and where it still reflects OOP habits.”

## Self-Review Checklist

- Can I describe the app without using the word “class”?
- Can I point to where data is modeled instead of implied?
- Can I explain why borrowing appears in the current API?

## How An Expert Rust Developer Thinks About This

An expert sees the current app as a set of typed boundaries: validated inputs, calculated schedule, formatted outputs. They look for data flow clarity before abstraction opportunities.

## Suggested Refactors To The Real App

- add a small architecture note in the repository root explaining the current module boundaries
- tighten argument parsing in `main` so input handling becomes an explicit boundary
