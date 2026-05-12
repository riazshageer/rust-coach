# Session 08: Module Organization

## Learning Objectives

- organize modules by responsibility rather than imported architecture diagrams
- keep modules cohesive and teachable
- understand when to split and when to stop splitting

## Rust Philosophy Discussion

Modules are not just folders. They are a visibility and responsibility tool. Good module organization reduces accidental coupling and makes invariants easier to locate.

## Practical Code Walkthroughs

- `src/app/`
- `src/domain/`
- `src/services/`
- `src/formatting/`
- `src/errors/`

## Refactoring Exercises

- explain the repository in five sentences, one per top-level module area
- identify one possible future split and one split that would be premature

## Architectural Reasoning

This repository avoids a giant `lib.rs` or `main.rs` because the learning goal is explicit structure. At the same time, it avoids deep nesting because the app is still small.

## Ownership Analysis

Module boundaries often simplify ownership because the code is forced to decide which layer owns which concepts. That reduces ad hoc data movement.

## Common Beginner Mistakes

- one huge file
- too many tiny files with no meaningful separation
- naming modules after generic patterns instead of responsibilities

## OOP-Thinking Traps

- copying controller-service-repository layering from other ecosystems
- assuming every level must talk only to the next level regardless of practicality

## Idiomatic Rust Alternatives

- organize by concern and change rate
- keep domain terms close together
- place conversions near boundaries that need them

## Challenge Exercises

- add a new CLI-oriented module if argument parsing grows
- write a short note on why `errors` is a top-level concern in this app

## Suggested Follow-Up Prompts For Codex

- “Review this module organization and tell me whether the current boundaries are cohesive or merely tidy-looking.”

## Self-Review Checklist

- Can I explain why each module exists?
- Would a new contributor know where to add a feature?
- Did I split for clarity or for aesthetics only?

## How An Expert Rust Developer Thinks About This

An expert organizes code around responsibility, boundary pressure, and change patterns. They know the best structure is often boring and obvious in hindsight.

## Suggested Refactors To The Real App

- add a dedicated CLI input module once argument parsing becomes more than one flag
- avoid splitting formatters further until another presentation concern appears
