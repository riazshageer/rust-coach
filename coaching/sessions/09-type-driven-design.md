# Session 09: Type-Driven Design

## Learning Objectives

- use types to carry architectural meaning
- detect missing types by reading repeated conditionals and vague strings
- distinguish useful types from ceremonial wrappers

## Rust Philosophy Discussion

Type-driven design in Rust is practical, not mystical. If the compiler can help enforce a rule cheaply and clearly, let it.

## Practical Code Walkthroughs

- `src/domain/location.rs`
- `src/app/config.rs`
- `src/main.rs`

## Refactoring Exercises

- identify one string or primitive in the app that wants to become a type
- design the smallest type that improves correctness

## Architectural Reasoning

`AppConfig` makes the startup boundary explicit. `Location` makes the calculation input explicit. Both reduce the number of “just pass some values around” interactions.

## Ownership Analysis

Type-driven design often improves ownership because it groups related data into cohesive values with natural movement boundaries.

## Common Beginner Mistakes

- making types for everything even when they add no meaning
- using plain strings to represent closed sets
- leaving invalid combinations to runtime checks scattered across functions

## OOP-Thinking Traps

- creating class-shaped wrappers with trivial methods
- treating types as containers only, not as invariants

## Idiomatic Rust Alternatives

- small enums for closed choices
- smart constructors for validated values
- composition of meaningful types instead of primitive bundles

## Challenge Exercises

- introduce a validated output-format type or CLI command type
- model config-file input separately from validated runtime config if you add deserialization

## Suggested Follow-Up Prompts For Codex

- “Find the next useful type in this repository and justify it like a Rust architect.”

## Self-Review Checklist

- What rule does this type enforce?
- What ambiguity does this type remove?
- Could I explain the benefit without hand-waving?

## How An Expert Rust Developer Thinks About This

An expert looks for pressure points where a type can eliminate a whole class of confusion. They also know when a primitive is already good enough.

## Suggested Refactors To The Real App

- model CLI output selection more strongly
- add input DTOs if configuration deserialization is introduced
