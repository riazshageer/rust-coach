# Rust Coaching System

This repository is both a working Rust application and a long-term apprenticeship environment.

The prayer-times app is the anchor project. Every coaching artifact in this folder points back to the real code in `src/` so learning stays concrete, architectural, and cumulative.

## What This Repository Teaches

- Idiomatic Rust instead of Rust-flavored OOP
- Type-driven design instead of stringly typed glue
- Ownership-aware architecture instead of accidental cloning
- Domain modeling instead of primitive obsession
- Iterator-oriented transformations instead of repetitive procedural loops
- Pragmatic modularity instead of overengineered layering
- Structured error design instead of vague generic failures
- Capability-oriented traits instead of abstraction for its own sake

## How To Use This Folder

1. Read [learning-path.md](./learning-path.md) to understand the progression.
2. Work through the session files in `coaching/sessions/` in order.
3. Use the prompts in `coaching/codex/` when pairing with Codex or another coding agent.
4. Use the checklists in `coaching/reviews/` before and after each change.
5. Pick exercises from `coaching/exercises/` that directly evolve the real app.
6. Revisit `coaching/progression/` whenever you feel stuck between levels.

## Coaching Philosophy

This system is intentionally opinionated.

- Prefer explicit code over magic.
- Prefer small, cohesive types over giant manager objects.
- Prefer compile-time guarantees over runtime cleanup.
- Prefer composition over inheritance-shaped architecture.
- Prefer borrowing over cloning when the lifetime model is manageable.
- Prefer one justified abstraction over five speculative ones.

## Current Application Shape

The app already demonstrates a healthy baseline:

- `app/` orchestrates work
- `domain/` owns validated business concepts
- `services/` wrap integration logic
- `formatting/` handles presentation
- `errors/` centralize typed failures

That structure is not the final destination. It is the starting architecture for progressive refinement.

## Suggested Workflow For Each Session

1. Read the session.
2. Open the relevant source files mentioned in the walkthrough.
3. Ask Codex to review the current state using one of the prompts.
4. Make the smallest meaningful change.
5. Run checks and note what the compiler taught you.
6. Self-review using the checklist attached to the session.
7. Write down what changed in your mental model, not just in the code.

## Rules For AI Pairing

- Ask the AI to explain tradeoffs, not just generate code.
- Ask for architecture reasoning before implementation.
- Ask what not to abstract yet.
- Ask for ownership analysis when code feels awkward.
- Push back on unnecessary traits, lifetimes, generics, `Arc`, `Mutex`, or async.
- Keep the application real and maintainable.

## Success Criteria

You are progressing well when:

- You can explain why a type exists.
- You notice invalid states before you write the code.
- You can smell procedural drift in a design.
- You use iterators because they clarify transformation, not because they look advanced.
- You let the borrow checker shape architecture instead of fighting it line by line.

This folder is designed to be revisited. The goal is not speed. The goal is a professional Rust way of thinking.
