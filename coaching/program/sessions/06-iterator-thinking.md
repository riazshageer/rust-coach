# Session 06: Iterator Thinking

## Outcome

Recognize when data transformation should be expressed as a pipeline instead of a procedural loop.

## What This Session Is Really Teaching

Iterators are not about cleverness. They are about making data flow, intent, and intermediate states easier to read.

## Source Focus

- `src/formatting/terminal_formatter.rs`
- `src/formatting/json_formatter.rs`

## Work To Attempt

1. Find one area where the code is mostly transforming data.
2. Decide whether an iterator pipeline would clarify it or make it harder to read.
3. Explain the tradeoff before changing anything.

## Exit Criteria

- you can identify transformation-heavy code
- you can explain why an iterator chain is or is not justified
- you can prefer clarity over novelty
