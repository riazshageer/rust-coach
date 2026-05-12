# Learning Path

This learning path turns the prayer-times application into a structured Rust architecture apprenticeship.

## Phase 1: Rust Thinking

Sessions `01` through `06`

Focus:

- thinking in data and transformations
- modeling real domain concepts
- using smart constructors
- building typed errors
- understanding ownership through design
- replacing repetitive control flow with iterator pipelines

Expected outcome:

You stop translating Java, C#, or TypeScript designs directly into Rust.

## Phase 2: Architectural Fluency

Sessions `07` through `12`

Focus:

- capability-oriented traits
- module organization
- type-driven design
- testing strategies
- serialization and configuration boundaries
- CLI architecture

Expected outcome:

You can evolve the application without reaching for unnecessary framework-style patterns.

## Phase 3: Systems Thinking

Sessions `13` through `17`

Focus:

- concurrency fundamentals
- when async is justified
- zero-cost abstractions
- pragmatic clean architecture
- extensible system design

Expected outcome:

You can reason about scaling the app without jumping to cargo-cult abstractions.

## Phase 4: Senior Rust Design

Sessions `18` through `20`

Focus:

- API design
- real-world Rust patterns
- final architecture review

Expected outcome:

You can explain design decisions like an architect, not just implement features like a task executor.

## Repeating Loop

After each phase:

1. Review current source layout.
2. Run one architecture review prompt.
3. Complete one real-app exercise.
4. Capture one anti-pattern you almost introduced.
5. Re-explain the current design in your own words.

## Level Markers

Beginner-to-intermediate signs:

- You stop using `String` for everything.
- You separate domain concepts from library types.
- You know when a constructor should return `Result`.

Intermediate-to-advanced signs:

- You can justify or reject a trait based on capabilities.
- You intentionally design APIs around borrowing and ownership.
- You use tests to lock architecture, not just behavior.

Architect mindset signs:

- You can improve design while reducing complexity.
- You can tell which boundaries belong to the app and which belong to dependencies.
- You know what not to generalize yet.
