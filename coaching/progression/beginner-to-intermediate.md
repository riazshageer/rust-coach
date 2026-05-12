# Beginner To Intermediate

This transition is about moving from syntax acquisition to architectural intention.

## Skills To Build

- model simple domain concepts as types
- use `Result` for validated construction
- understand when ownership should move and when borrowing should stay
- replace repeated printing or mapping code with iterator pipelines
- separate orchestration from domain rules

## Signs You Are Still Early

- you use `String` when an enum or typed value would be clearer
- you reach for `clone()` before redesigning a function boundary
- you create wrappers without invariants
- you want classes more than data models

## Practices To Repeat

- read compiler errors slowly
- ask what the type should be before writing the function
- explain each module in one sentence
- run a review checklist after each refactor

## Repository Milestones

- add one more validated domain type
- add tests for smart constructors
- extend formatting without weakening architecture
- add explicit config loading instead of hardcoding everything in `main`
