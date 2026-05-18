# Ownership Review Prompt

```text
Review my code through the lens of ownership and borrowing.

Look for:

- unnecessary cloning
- unclear lifetimes caused by design shape
- ownership moving through the wrong boundary
- borrowed capability opportunities
- places where a simpler data flow would eliminate borrow friction
- whether owning data would simplify the use-case boundary

Rules:

- do not rewrite the code for me
- findings first
- explain the ownership story in concrete terms, not generic Rust slogans

Response format:

1. Ownership findings
2. The core ownership story of this code
3. The single best improvement I should attempt next
```
