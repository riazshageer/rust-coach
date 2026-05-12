# Session 02: Domain Modeling

## Learning Objectives

- understand why `Location` is better than a coordinate tuple
- separate domain meaning from third-party crate types
- spot primitive obsession early

## Rust Philosophy Discussion

Good Rust domain modeling reduces ambiguity. If a value matters to the business rules, it deserves a type that expresses meaning and constraints.

## Practical Code Walkthroughs

- `src/domain/location.rs`
- `src/app/config.rs`
- `src/services/prayer_service.rs`

## Refactoring Exercises

- write down which values in the app are still “just primitives”
- decide whether each primitive should remain primitive or become a domain type

## Architectural Reasoning

The domain layer should speak the language of the application, not the language of the dependency. `salah::Coordinates` is useful, but it is an integration type, not the app’s conceptual source of truth.

## Ownership Analysis

`Location` is `Copy`, which is appropriate because it is small and value-like. This is not performance theater. It matches the semantics of a tiny validated value object.

## Common Beginner Mistakes

- wrapping primitives without adding invariants
- exposing raw dependency types everywhere
- putting validation in unrelated service functions

## OOP-Thinking Traps

- using a “LocationManager” instead of a strong `Location` type
- adding setters that can break invariants after construction

## Idiomatic Rust Alternatives

- use newtypes where the domain rule matters
- centralize validation in constructors
- convert to library types only at the boundary that needs them

## Challenge Exercises

- decide whether output format should become a small enum type in the app boundary
- decide whether a future config file should deserialize directly into `AppConfig` or into an intermediate input type

## Suggested Follow-Up Prompts For Codex

- “Review the domain model and identify where primitive obsession still exists in this app.”

## Self-Review Checklist

- Does each domain type carry meaning?
- Are invariants attached to construction?
- Am I leaking library concepts into domain code unnecessarily?

## How An Expert Rust Developer Thinks About This

An expert treats domain modeling as architecture, not polish. They know the right type often removes several future conditionals.

## Suggested Refactors To The Real App

- add tests around invalid coordinates and decide how to handle NaN and infinity
- introduce an explicit output-format domain or application input type
