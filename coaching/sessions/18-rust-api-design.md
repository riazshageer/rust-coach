# Session 18: Rust API Design

## Learning Objectives

- design ergonomic function signatures
- balance ownership, clarity, and future flexibility
- make the right thing the easy thing for callers

## Rust Philosophy Discussion

Rust API design is about more than type correctness. It is about expressing intent clearly, making misuse harder, and keeping signatures honest about cost and ownership.

## Practical Code Walkthroughs

- `src/app/prayer_app.rs`
- `src/app/config.rs`
- `src/domain/location.rs`

## Refactoring Exercises

- review each public function and ask whether its name, parameters, and return type match what it really does
- identify one API that could become clearer with a different input or output type

## Architectural Reasoning

Public APIs in this app are small, which is good. Small public surfaces are easier to reason about, document, and evolve carefully.

## Ownership Analysis

API signatures communicate cost. Taking ownership means something. Returning `String` means something. Borrowing a formatter means something. In Rust, signatures are architecture made visible.

## Common Beginner Mistakes

- exposing fields or functions too widely
- using vague names like `process` or `handle`
- forcing ownership when borrowing would communicate intent better

## OOP-Thinking Traps

- mirroring class-method naming habits
- treating public APIs as mutable object surfaces

## Idiomatic Rust Alternatives

- name functions after domain actions or outputs
- keep public surfaces narrow
- let types and signatures teach the caller what to expect

## Challenge Exercises

- review whether `render_today` and `schedule_for_today` are the right names
- consider whether a default config constructor would improve the public API

## Suggested Follow-Up Prompts For Codex

- “Review the public APIs in this repository for ergonomic Rust design and teach me what the signatures are saying.”

## Self-Review Checklist

- Can a caller misuse this API easily?
- Does the name match the behavior?
- Does ownership in the signature match reality?

## How An Expert Rust Developer Thinks About This

An expert reads APIs from the caller’s perspective first. They optimize for honest, unsurprising use.

## Suggested Refactors To The Real App

- add explicit default constructors or parsing APIs if they make startup clearer
- tighten public surfaces to the functions actually needed across modules
