# Session 07: Traits And Capabilities

## Learning Objectives

- understand when a trait is justified
- treat traits as capability boundaries, not interface rituals
- compare traits with enums and concrete functions

## Rust Philosophy Discussion

In Rust, traits are most useful when callers need behavior without caring about a specific concrete type. They are weaker when introduced only to satisfy an architectural fashion.

## Practical Code Walkthroughs

- `src/formatting/mod.rs`
- `src/formatting/terminal_formatter.rs`
- `src/formatting/json_formatter.rs`

## Refactoring Exercises

- explain why `PrayerFormatter` exists and what would happen without it
- identify one area in the app where a trait would currently be premature

## Architectural Reasoning

The formatter trait is justified because the app can render the same schedule in multiple ways while depending on a single capability: formatting a schedule into a string.

## Ownership Analysis

Borrowing a trait-bound formatter keeps the call site flexible without forcing ownership or dynamic dispatch complexity beyond what is needed.

## Common Beginner Mistakes

- adding traits for every struct
- hiding single implementations behind trait objects too early
- using traits to imitate abstract base classes

## OOP-Thinking Traps

- “Every service should have an interface”
- “Traits make code more professional by default”

## Idiomatic Rust Alternatives

- use a concrete type when there is only one real behavior
- use an enum when the set of variants is closed
- use a trait when the capability is the abstraction that matters

## Challenge Exercises

- decide whether CLI parsing should use a trait, enum, or simple function
- evaluate whether future config loading needs a trait or just conversion functions

## Suggested Follow-Up Prompts For Codex

- “Review the traits in this repository and tell me which are justified capabilities and which would be overengineering.”

## Self-Review Checklist

- Why does this trait exist?
- Who benefits from the abstraction?
- Is there real variation in behavior?

## How An Expert Rust Developer Thinks About This

An expert resists speculative traits. They wait for real pressure and then choose the lightest abstraction that captures the capability cleanly.

## Suggested Refactors To The Real App

- keep the formatter trait
- avoid introducing service traits until testing or extension pressure makes them clearly useful
