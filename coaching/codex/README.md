# Prompt Index

These prompts are helper prompts for the coaching system. They reduce startup friction, but the roadmap remains the source of truth.

## Start Or Resume Work

- `session-kickoff-prompt.md`

Use when beginning a session or resuming after a restart.

## Ask For The Next Coding Move

- `master-coach-prompt.md`

Use when you want the coach to identify the best next slice from the roadmap and current repo state.

## Ask For Hints Without Handing Off The Code

- `hint-only-prompt.md`

Use when you want guidance but still intend to implement the change yourself.

## Recover When Stuck

- `stuck-prompt.md`

Use when you need diagnosis and scope reduction.

## Request A Findings-First Review

- `review-prompt.md`

Use after you have a meaningful diff.

## Pressure-Test Architecture

- `architecture-review-prompt.md`

Use when a change introduces or alters boundaries.

## Pressure-Test Ownership Choices

- `ownership-review-prompt.md`

Use when the data flow feels clone-heavy, borrow-heavy, or unclear.

## Pressure-Test Transformation Code

- `iterator-review-prompt.md`

Use when collection-shaping logic feels noisy or procedural.

## Stage A Refactor Safely

- `refactor-prompt.md`

Use when the code shape should change and you want a reviewable sequence.

## Generate The Next Stretch Task

- `challenge-generator-prompt.md`

Use after a slice lands and you want the next deliberate stretch.

## Close The Session

- `session-close-prompt.md`

Use before stopping so the coach updates the restart state and next step.
