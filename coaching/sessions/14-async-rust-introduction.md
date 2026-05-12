# Session 14: Async Rust Introduction

## Learning Objectives

- understand what async is actually for
- compare async with plain synchronous IO and threads
- avoid introducing async where it does not belong

## Rust Philosophy Discussion

Async is not “modern Rust mode.” It is a tool for managing many in-flight IO operations efficiently. If the app is CPU-light and local, sync code is often the better architecture.

## Practical Code Walkthroughs

- `src/main.rs`
- future configuration or remote-location boundaries you may add later

## Refactoring Exercises

- write a short architecture note explaining why the current app does not need async
- sketch one future feature that might justify it

## Architectural Reasoning

The current prayer-times app is synchronous because its work is local, bounded, and simple. That is the correct design today. Async should enter only if the repository later gains real external IO pressure.

## Ownership Analysis

Async affects ownership because values often need to cross await points or task boundaries. That raises complexity, so the value must justify the cost.

## Common Beginner Mistakes

- introducing async for style points
- mixing sync and async boundaries without a reasoned plan
- assuming async always means faster

## OOP-Thinking Traps

- layering async everywhere because a framework would do it
- abstracting executors and runtimes before the use case exists

## Idiomatic Rust Alternatives

- stay synchronous until remote or high-latency IO appears
- isolate any future async boundary carefully
- keep domain logic sync even if an outer adapter becomes async

## Challenge Exercises

- propose a remote configuration or geocoding feature
- compare sync blocking, threads, and async before coding

## Suggested Follow-Up Prompts For Codex

- “Evaluate whether async belongs in this repository yet, and explain the reasoning like a Rust architect.”

## Self-Review Checklist

- Is async solving an actual boundary problem?
- Does the complexity cost pay for itself?
- Can the domain stay synchronous?

## How An Expert Rust Developer Thinks About This

An expert treats async as an integration concern, not a whole-architecture identity.

## Suggested Refactors To The Real App

- add an architecture note for future async evaluation
- keep current code synchronous
