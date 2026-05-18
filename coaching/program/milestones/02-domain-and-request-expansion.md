# Milestone 2: Domain And Request Expansion

## Objective

Grow from one hard-coded use case into a small scheduling product.

## Required Coding Work

1. Support schedule queries for an explicit date.
2. Support schedule queries for a date range.
3. Add a next-prayer query relative to a point in time.
4. Introduce explicit calculation profile selection.
5. Decide which request objects belong in domain versus application.

## Stretch Work

- design a reusable schedule collection type
- evaluate whether local and UTC time views should both be first-class in the model

## Rust Concepts Under Pressure

- ownership across request flows
- iterator pipelines
- date and time handling
- API design

## Decision Questions For The Coach

- Is the domain exposing raw library types too early?
- Should a range query stream, allocate a collection, or support both?
- What type best represents "next prayer" without leaking formatting concerns?

## Done Means

- at least three real use cases exist
- inputs are explicit instead of hidden inside orchestration
- the learner can explain why each type lives where it does
