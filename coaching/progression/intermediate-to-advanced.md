# Intermediate To Advanced

This transition is about turning local Rust skill into system design skill.

## Skills To Build

- capability-oriented trait design
- ergonomic API design around ownership
- principled testing strategy
- explicit boundary management between domain and dependencies
- practical concurrency reasoning
- deciding when async is worth its cost

## Common Plateau

Intermediate developers often know many Rust features but still design reactively. The next step is learning to shape the code so the compiler has less chaos to police.

## Practices To Repeat

- justify every new abstraction in one sentence
- compare borrowed and owned API versions before committing
- review whether an enum is better than a trait
- inspect allocations and clones in hot or repeated paths

## Repository Milestones

- introduce configuration loading from serialized input
- grow the CLI without turning `main` into a command jungle
- add well-scoped concurrency or async only where there is a real use case
- perform a full architecture review and remove one unnecessary abstraction
