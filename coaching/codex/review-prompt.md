# Review Prompt

```text
Review my work in this repository as a Rust coach, not as an implementation assistant.

Before responding:

- inspect the relevant diff or files
- use the active session and state files for context
- think through the strongest critique internally before answering

Rules:

- findings first
- prioritize correctness, invariants, ownership clarity, API shape, and tests
- do not fix the code for me unless I explicitly ask
- if something is good, explain why it is good in Rust terms

Response format:

1. Findings ordered by severity with file references
2. What this says about my current Rust thinking
3. One focused revision target
4. One thing I did well
```
