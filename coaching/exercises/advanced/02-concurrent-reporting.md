# Advanced Exercise 02: Add Concurrent Reporting Carefully

## Goal

Explore concurrency only where it adds real value.

## Change Request

- Add a feature that computes reports for multiple locations.
- Start with a sequential design, then evaluate whether threads improve the use case.
- If concurrency is added, explain the ownership model explicitly.

## Why This Matters

This teaches that concurrency in Rust starts with data ownership and work partitioning, not with `Arc<Mutex<T>>`.

## Constraints

- No shared mutable state unless truly necessary.
- Prefer message passing or independent owned work units.
- Measure clarity before cleverness.
