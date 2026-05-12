# Session 13: Concurrency Foundations

## Learning Objectives

- understand concurrency as ownership plus work partitioning
- recognize when the app could use parallelism
- avoid shared mutable state as the default strategy

## Rust Philosophy Discussion

Rust concurrency is powerful because ownership rules make races harder to express. That power is best used by designing independent work units, not by reaching immediately for shared state wrappers.

## Practical Code Walkthroughs

- `src/services/prayer_service.rs`
- `src/domain/location.rs`

## Refactoring Exercises

- imagine computing schedules for multiple locations
- design a sequential API first, then ask whether parallel execution adds value

## Architectural Reasoning

A multi-location report would likely work best as a collection of owned input values transformed into owned results. That is a concurrency-friendly architecture because each unit of work is isolated.

## Ownership Analysis

Concurrency-friendly design usually starts with owned messages or tasks that can move safely between threads. Borrow-heavy cross-thread designs are often the wrong starting point.

## Common Beginner Mistakes

- adding `Arc<Mutex<T>>` before proving shared mutation is needed
- conflating concurrency with asynchrony
- parallelizing small tasks with no user benefit

## OOP-Thinking Traps

- central manager objects coordinating mutable shared state
- hidden global state to make thread access easier

## Idiomatic Rust Alternatives

- independent owned work items
- message passing or result collection
- explicit thread boundaries

## Challenge Exercises

- design a `Vec<Location>` reporting feature
- compare sequential and thread-per-location versions on clarity first

## Suggested Follow-Up Prompts For Codex

- “Show me how to think about concurrency in this app without defaulting to Arc and Mutex.”

## Self-Review Checklist

- Did I design independent work first?
- Is concurrency solving a real problem?
- Could simpler sequential code still be better?

## How An Expert Rust Developer Thinks About This

An expert asks what can be owned independently and processed independently. If the answer is “almost everything,” concurrency may be straightforward. If not, the architecture may need work first.

## Suggested Refactors To The Real App

- add a multi-location report exercise before implementing any threaded version
- document why the current single-location design is intentionally synchronous
