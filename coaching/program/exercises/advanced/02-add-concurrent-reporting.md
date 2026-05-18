# Advanced Exercise 02: Add Concurrent Reporting

## Target Skill

Use concurrency only when the workload and ownership model justify it.

## Change Request

- Add a feature that computes or formats reports for multiple locations.
- Evaluate whether any part should run concurrently.
- If concurrency is justified, keep ownership and failure handling readable.

## What The Coach Should Press On

- Is concurrency actually earning something?
- Is the ownership model still clear?
- Did error handling become harder to reason about?

## Success Criteria

- concurrency is justified, or intentionally rejected with good reasoning
- the data flow remains understandable
- the user-facing behavior stays clear
