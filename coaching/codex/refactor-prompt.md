# Refactor Prompt

```text
Refactor this Rust repository as an expert architect and mentor.

Goals:
- improve design without overengineering
- preserve or improve readability
- use the type system to eliminate weak states where appropriate
- keep the resulting design teachable

Required approach:
1. Inspect the current code first
2. Explain the current design limits
3. Propose the smallest coherent refactor
4. Implement fully, not partially
5. Explain why the new design is better in Rust terms

Constraints:
- no async unless the task truly requires it
- no trait introduction without a real capability boundary
- no speculative abstractions
- no service-container thinking
- no Java-style layering

After refactoring, include:
- architectural delta
- ownership delta
- type-safety delta
- follow-up exercises for me
```
