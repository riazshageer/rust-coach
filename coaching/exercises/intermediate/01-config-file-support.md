# Intermediate Exercise 01: Add Config File Support

## Goal

Introduce serialized configuration while preserving architecture clarity.

## Change Request

- Allow loading `AppConfig` data from a JSON file.
- Keep validation in domain types rather than trusting deserialization blindly.
- Decide where dependency-specific parsing should live.

## Why This Matters

This exercise teaches boundary design between serialized input, validated domain data, and application orchestration.

## Constraints

- Avoid turning domain types into serialization dumping grounds if that weakens invariants.
- Prefer one clear conversion path.
- Keep error types informative.

## Expert Hint

A common Rust architecture move is to deserialize into an input DTO and then convert into validated domain types.
