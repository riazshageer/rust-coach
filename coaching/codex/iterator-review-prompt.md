# Iterator Review Prompt

```text
Review this code for iterator thinking and transformation clarity.

Focus on:

- repetitive control flow that should become a transformation pipeline
- iterator use that hurts readability instead of improving it
- temporary allocations that are not earning their keep
- naming that hides the shape of the data flow

Rules:

- do not chase cleverness
- prefer readable iterator pipelines over procedural repetition
- do not implement the final rewrite for me unless I ask

Response format:

1. Findings
2. Where the data flow is clear
3. Where it is procedural or noisy
4. The next refactor worth trying
```
