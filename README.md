# Rust Prayer Times Coaching Repository

This repository is a hands-on Rust coaching system built around a real application. The coachee is expected to code continuously, ship small slices, defend design choices, and grow into production-grade Rust engineering judgment.

The application in `src/` starts small on purpose. The coaching system in `coaching/` turns it into a staged delivery program for an enterprise-style product: a prayer-times engine with clear architecture, strong domain modeling, test coverage, operational thinking, and disciplined tradeoff analysis.

## What This Repository Optimizes For

- coding, not passive discussion
- coach-guided delivery on a real codebase
- clean architecture without cargo-cult layering
- git history as evidence of design thinking
- low-friction agent memory so the learner is not manually carrying session state

## Repository Map

- `src/`: the application under construction
- `coaching/program/`: product vision, target architecture, roadmap, and milestone briefs
- `coaching/codex/`: helper prompts for kickoff, review, architecture checks, and recovery when stuck
- `coaching/ops/`: how the coaching workflow operates
- `coaching/state/`: checked-in baseline state for the program
- `coaching/state/local/`: git-ignored agent-managed working memory, created automatically
- `scripts/coach_context.sh`: context snapshot for the coach
- `scripts/init_local_coaching_state.sh`: bootstraps local memory files if missing

## How Work Starts

1. Run `scripts/coach_context.sh`.
2. Start the agent with `coaching/codex/session-kickoff-prompt.md`.
3. Let the coach rebuild context from repo files, git state, and local memory.
4. Code the next backlog slice.
5. Ask for review after a meaningful diff.
6. Let the coach update local memory before ending the session.

The learner should not have to remember where things were left. The agent owns the rolling context in `coaching/state/local/`.

## Product Target

The end-state product is not just "print today's prayer times." The target is a production-grade prayer-times platform with:

- explicit request models and invariants
- CLI commands and configuration profiles
- multiple output adapters
- testable application boundaries
- clear error taxonomy
- concurrency and async evaluated intentionally, not by default
- packaging, observability, and deployment considerations

See [coaching/program/product-vision.md](/home/rshageer/Playground/prayer_times/coaching/program/product-vision.md:1), [coaching/program/target-architecture.md](/home/rshageer/Playground/prayer_times/coaching/program/target-architecture.md:1), and [coaching/program/delivery-roadmap.md](/home/rshageer/Playground/prayer_times/coaching/program/delivery-roadmap.md:1).

## Current App Snapshot

Today the app:

- calculates prayer times for a hard-coded location
- renders terminal or JSON output
- has a small but clean module split
- has no tests yet
- has not yet grown into a production architecture

That is deliberate. The coaching program uses this gap as the delivery surface.

## Running The App

```bash
cargo run
```

JSON output:

```bash
cargo run -- --json
```

Checks:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features
```

## Start Here

- Coaching overview: [coaching/README.md](/home/rshageer/Playground/prayer_times/coaching/README.md:1)
- Program overview: [coaching/program/README.md](/home/rshageer/Playground/prayer_times/coaching/program/README.md:1)
- Delivery roadmap: [coaching/program/delivery-roadmap.md](/home/rshageer/Playground/prayer_times/coaching/program/delivery-roadmap.md:1)
- Current work tracker: [coaching/state/current-session.md](/home/rshageer/Playground/prayer_times/coaching/state/current-session.md:1)
- Prompt index: [coaching/codex/README.md](/home/rshageer/Playground/prayer_times/coaching/codex/README.md:1)
