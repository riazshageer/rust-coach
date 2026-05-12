# Intermediate Exercise 02: Add Current And Next Prayer Reporting

## Goal

Extend the real app with more useful information while keeping the domain model coherent.

## Change Request

- Add a feature that reports the current prayer and next prayer for today.
- Decide whether these concepts belong in the domain model, the service layer, or formatting only.
- Keep the formatter output clean for both terminal and JSON.

## Why This Matters

This teaches feature growth without collapsing boundaries.

## Review Questions

- Did I leak `salah` details through my app boundary?
- Did I add data to the right model?
- Is the feature discoverable without making the formatting layer responsible for business logic?
