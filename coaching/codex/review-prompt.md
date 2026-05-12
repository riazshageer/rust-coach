# Review Prompt

Use this when you want a high-signal Rust review focused on correctness and idiomatic design.

```text
Review this repository change like an expert Rust architect.

Review priorities:
1. Bugs and behavioral risks
2. Ownership and borrowing quality
3. Domain modeling quality
4. Error-handling quality
5. Abstraction quality
6. Idiomatic Rust style

Repository context:
- The codebase is a prayer-times application used as a Rust coaching repository.
- Feedback should teach, not just judge.
- Use the real architecture in the repository as context.

Important constraints:
- Do not praise vaguely.
- Do not default to stylistic nits.
- Do not suggest abstractions without a strong justification.
- Call out OOP-shaped thinking when present.
- Identify unnecessary cloning, weak types, trait abuse, and procedural drift.

Output format:
1. Findings ordered by severity, with file references
2. Why each issue matters in Rust specifically
3. Suggested fix direction
4. A short “what this teaches” section
```
