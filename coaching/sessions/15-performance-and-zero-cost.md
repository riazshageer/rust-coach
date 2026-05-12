# Session 15: Performance And Zero-Cost

## Learning Objectives

- think clearly about performance without premature optimization
- understand what zero-cost abstraction means in practice
- inspect allocation and copying decisions pragmatically

## Rust Philosophy Discussion

Zero-cost abstractions are abstractions that compile away to efficient code without hiding runaway costs. The phrase is not a license to micro-optimize everything.

## Practical Code Walkthroughs

- `src/domain/location.rs`
- `src/domain/prayer_time.rs`
- `src/formatting/json_formatter.rs`

## Refactoring Exercises

- identify where the app allocates strings intentionally
- identify where allocation could be reduced and decide whether the change is worth it

## Architectural Reasoning

The app already uses small value types and straightforward transformations. That is a good baseline. Performance work should preserve readability unless evidence shows a hot path.

## Ownership Analysis

Owned strings in formatter output are appropriate because the final output must exist independently. Borrowing everywhere just to avoid allocation would complicate the design for little gain.

## Common Beginner Mistakes

- replacing readable code with lifetime-heavy code too early
- assuming `Copy` or iterators automatically make code fast
- ignoring allocation entirely because Rust feels low-level

## OOP-Thinking Traps

- building abstraction layers that allocate or box per call without noticing
- hiding data flow so costs become hard to reason about

## Idiomatic Rust Alternatives

- keep data movement explicit
- optimize when the boundary or repetition justifies it
- use concrete simple code until evidence pushes otherwise

## Challenge Exercises

- inspect whether formatter output can reduce duplication cleanly
- measure before and after if you optimize anything repeated

## Suggested Follow-Up Prompts For Codex

- “Review this code for practical performance issues and separate real costs from premature-optimization temptations.”

## Self-Review Checklist

- Did I optimize a real cost?
- Did readability survive?
- Can I explain the tradeoff?

## How An Expert Rust Developer Thinks About This

An expert values predictable costs and clear ownership. They optimize where data size, frequency, or latency gives them a reason.

## Suggested Refactors To The Real App

- only optimize repeated formatting or collection flows if profiling or scale justifies it
- keep value types small and explicit
