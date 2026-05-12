# Ownership Review Prompt

```text
Review this Rust code specifically through the lens of ownership and borrowing.

Look for:
- unnecessary clones
- temporary ownership patterns that could be borrowing
- APIs that force allocation unnecessarily
- return types that own more than they should
- lifetime complexity that indicates poor architecture
- places where a different data flow would simplify borrowing

Teaching emphasis:
- explain how ownership concerns should influence architecture
- distinguish between justified ownership and accidental ownership
- show how to think before reaching for `.clone()`

Constraints:
- do not recommend borrowing if it makes the code brittle or unreadable
- do not introduce lifetime-heavy APIs unless the benefit is clear
- keep suggestions pragmatic and educational

End with:
- most important ownership improvement
- one thing the code already does well
- one ownership habit I should practice
```
