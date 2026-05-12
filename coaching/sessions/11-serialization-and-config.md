# Session 11: Serialization And Config

## Learning Objectives

- understand serialized input as an application boundary
- keep validation separate from raw deserialization
- design configuration flow without weakening domain rules

## Rust Philosophy Discussion

Serialization formats are transport concerns, not domain truth. Rust architecture improves when you treat deserialized input as something to validate and convert, not something to trust.

## Practical Code Walkthroughs

- `src/app/config.rs`
- `src/errors/mod.rs`
- `src/main.rs`

## Refactoring Exercises

- sketch a config-file input struct separate from `AppConfig`
- map how raw JSON would become validated domain values

## Architectural Reasoning

If this app gains config files, a good design is often: deserialize input -> validate into domain types -> construct `AppConfig` -> run app. That keeps external data from bypassing invariants.

## Ownership Analysis

Deserialized input is naturally owned data. Validation then converts it into stronger owned domain values that the app can use safely.

## Common Beginner Mistakes

- deriving deserialization directly on strong domain types without considering invariants
- mixing parsing logic into unrelated modules
- collapsing file IO, parsing, and validation into one function

## OOP-Thinking Traps

- creating a configuration service object before a simple conversion path is tried
- hiding config mutation behind setters

## Idiomatic Rust Alternatives

- use input DTOs at the boundary
- convert into validated domain types
- return typed errors that identify file, parse, or validation failure

## Challenge Exercises

- add JSON config-file loading
- keep the app startup path readable after adding it

## Suggested Follow-Up Prompts For Codex

- “Design a serialization and configuration boundary for this Rust app without weakening the domain model.”

## Self-Review Checklist

- Did untrusted input stay outside the domain until validation?
- Are config errors precise?
- Is startup flow still easy to follow?

## How An Expert Rust Developer Thinks About This

An expert separates outside-world representations from inside-world meaning. They do not let convenience at the serialization boundary corrupt the domain layer.

## Suggested Refactors To The Real App

- add a `config_input` type if file loading is introduced
- preserve `AppConfig` as the validated runtime configuration
