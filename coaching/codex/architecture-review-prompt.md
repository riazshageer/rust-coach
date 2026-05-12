# Architecture Review Prompt

```text
Perform an architecture review of this Rust repository.

Review for:
- boundary clarity between app, domain, services, formatting, and errors
- whether domain concepts are modeled strongly enough
- whether configuration and integration boundaries are explicit
- whether the module structure matches responsibility
- whether the current abstractions are justified
- whether any layering is accidental or cargo-culted

Teaching goals:
- explain how a Rust architect evaluates this design
- identify where the design is still procedural
- identify where the type system could carry more meaning
- identify what should stay simple

Constraints:
- no generic enterprise advice
- no framework-style architecture
- no unnecessary clean-architecture ceremony
- no suggestion to add traits, lifetimes, or generics unless they improve the actual code

End with:
- keep
- improve next
- avoid next
```
