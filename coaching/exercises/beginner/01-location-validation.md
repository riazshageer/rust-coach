# Beginner Exercise 01: Strengthen Location Validation

## Goal

Deepen the current `Location`, `Latitude`, and `Longitude` model.

## Change Request

- Add focused unit tests for valid and invalid coordinate construction.
- Decide whether NaN or infinite floating-point values should be accepted.
- If they should not be accepted, encode that rule in the smart constructors.

## Why This Matters

This exercise teaches that a domain type is only as strong as its invariants.

## Constraints

- Keep the API simple.
- Do not add traits unless they help the actual code.
- Prefer explicit validation logic over generic numeric abstractions.

## Expert Hint

A newtype without careful validation is often just renamed weakness.

## Self-Review

- Did I strengthen the invariant?
- Did I preserve a readable constructor API?
- Did the tests document the domain rule clearly?
