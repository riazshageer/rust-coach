# Session 16: Clean Architecture In Rust

## Learning Objectives

- distinguish useful boundary separation from ceremony
- adapt architecture ideas to Rust instead of copying them literally
- keep the repository teachable and concrete

## Rust Philosophy Discussion

Rust can support clean architectural boundaries, but not every clean-architecture diagram maps well to a small application. The language favors explicit concrete relationships over elaborate indirection.

## Practical Code Walkthroughs

- entire `src/` tree

## Refactoring Exercises

- identify which current boundaries are genuinely useful
- identify one enterprise-style pattern that would make this app worse today

## Architectural Reasoning

This repository already separates domain, application orchestration, services, formatting, and errors. That is enough clean architecture for the current size. Adding repositories, ports, adapters, and factories everywhere would mostly create indirection noise.

## Ownership Analysis

Too many layers often distort ownership. Data gets repeatedly converted, boxed, cloned, or hidden behind traits. Rust exposes that cost quickly, which is a reason to stay lean.

## Common Beginner Mistakes

- importing architecture names without architecture pressure
- building abstractions to impress rather than simplify
- wrapping all dependencies behind traits immediately

## OOP-Thinking Traps

- interface-per-layer
- use-case classes for tiny flows
- repository patterns without persistence complexity

## Idiomatic Rust Alternatives

- concrete modules with explicit boundaries
- small traits only where behavior variation is real
- direct composition through structs and functions

## Challenge Exercises

- write a one-page architecture summary of the current repository
- remove one abstraction from a hypothetical overengineered version and justify it

## Suggested Follow-Up Prompts For Codex

- “Evaluate this repository against clean architecture ideas, but adapt the advice to idiomatic Rust instead of enterprise boilerplate.”

## Self-Review Checklist

- Did I separate real concerns?
- Did I add indirection without payoff?
- Does the code remain concrete and navigable?

## How An Expert Rust Developer Thinks About This

An expert does not reject architecture. They reject ceremony disconnected from present needs.

## Suggested Refactors To The Real App

- preserve the current top-level boundaries
- resist repository/service explosion unless persistence or integrations truly arrive
