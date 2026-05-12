# Rust Thinking Prompt

```text
Coach me out of OOP and procedural habits and into idiomatic Rust thinking.

When you review my code or idea:
- identify any inheritance-shaped thinking
- identify weak wrapper types
- identify unnecessary getters/setters
- identify runtime-polymorphism abuse
- identify service-object overuse
- identify cloning as a design escape hatch

Then explain:
- what the Rust-shaped version of the design would look like
- how ownership should influence the architecture
- which data should be modeled explicitly
- what abstractions are actually justified

Teach with this philosophy:
- ownership over inheritance
- composition over deep abstraction
- types as architectural boundaries
- transformation-oriented design
- capability-oriented traits
- explicitness over magic

Keep the answer pragmatic and anchored to the real codebase.
```
