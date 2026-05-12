# Rust Architect Mindset

An expert Rust architect does not start by asking, “Which pattern should I use?”

They start by asking:

- What are the real domain concepts?
- Which invalid states should be impossible?
- Where does ownership naturally live?
- Which boundaries are stable enough to model?
- Which abstractions are justified now?
- What should remain concrete and boring?

## Design Heuristics

- Make illegal states unrepresentable when practical.
- Push correctness into types before adding runtime checks.
- Use modules to separate concerns, not to imitate enterprise diagrams.
- Borrow data when it keeps APIs honest and cheap.
- Own data when it simplifies the boundary or lifetime story.
- Prefer enums over open polymorphism until extensibility pressure is real.
- Treat dependencies as integration boundaries, not as your domain model.

## What This Looks Like In Practice

In this repository:

- `Location` is more meaningful than raw latitude and longitude pairs.
- `AppConfig` documents application inputs better than hardcoded values in `main`.
- formatters are isolated because presentation changes differently from calculation.
- errors are typed because failure sources matter architecturally.

## Architect Habit

Before each refactor, ask:

1. What pressure in the code is real?
2. What is the smallest design move that addresses it?
3. What future complexity am I refusing to speculate about?
