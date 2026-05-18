# Rust Architect Mindset

## The Mindset Shift

A strong Rust architect does not ask first, "How do I implement this?"

They ask:

- What are the real boundaries?
- Which states should be impossible?
- Who owns this data and why?
- Which abstraction is actually earning its keep?
- What is the cheapest design that preserves clarity and safety?

## Habits To Build

- model meaning before plumbing
- treat ownership as a design tool
- keep boundaries explicit
- use traits for capabilities, not decoration
- let tests and diffs reveal architectural quality

## Habits To Break

- class-first modeling
- speculative flexibility
- clone-first problem solving
- framework-shaped architecture without framework pressure
- assuming harder-looking code is more senior
