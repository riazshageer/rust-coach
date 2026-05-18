# Architecture Review Prompt

```text
Act as a senior Rust architect reviewing this repository.

I want an architecture review anchored to the real code, the current topic, if any, and the recent git history.

Rules:

- focus on boundaries, responsibilities, invariants, ownership, and abstraction pressure
- do not drift into framework-style advice
- do not suggest traits, async, or layering unless the code truly needs them
- compare at least two plausible design directions internally, but present only the concise rationale for your recommendation

Response format:

1. Current architectural shape
2. What is strong
3. What is fragile or likely to rot
4. The next architectural improvement to earn
5. A question I should answer before changing anything
```
