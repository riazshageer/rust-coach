# Milestone 6: Senior Engineering Judgment

## Objective

Practice hard engineering decisions with evidence.

## Required Coding Work

1. Evaluate whether batch schedule generation benefits from threads.
2. Evaluate whether async belongs anywhere in this product.
3. Review the architecture using git history and commit evolution.
4. Write a concise design defense for the current system.

## Stretch Work

- prototype a concurrent batch path behind a clear boundary
- document why async was rejected if it was rejected

## Rust Concepts Under Pressure

- concurrency
- async tradeoffs
- API stability
- architectural reasoning

## Decision Questions For The Coach

- Is there real parallel work here?
- Would async simplify anything, or only add lifecycle complexity?
- Which abstractions have actually paid rent?

## Done Means

- the learner can justify both technical choices and rejections
- the repo shows senior-level restraint, not just senior-level vocabulary
