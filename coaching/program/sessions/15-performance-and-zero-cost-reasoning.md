# Session 15: Performance And Zero-Cost Reasoning

## Outcome

Learn to reason about cost without turning the code into unreadable performance theater.

## What This Session Is Really Teaching

Zero-cost abstractions are earned when your design is both expressive and inexpensive. The point is not to micro-optimize blindly.

## Source Focus

- iterator-heavy code
- allocation boundaries
- formatting paths

## Work To Attempt

1. Identify one real allocation or ownership cost in the app.
2. Decide whether it matters.
3. Explain the readability tradeoff of optimizing it.

## Exit Criteria

- you can name one real cost in the code
- you can explain whether it matters at current scale
- you can avoid sacrificing clarity for imaginary wins
