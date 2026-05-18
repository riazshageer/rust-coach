# Learning Path

This program is delivery-first. The learner progresses by shipping milestones, not by advancing through abstract Rust topics.

## Phase 1: Foundation Hardening

Goal:
Turn the current toy CLI into a small but disciplined application.

Primary work:

- add tests around domain invariants
- replace ad-hoc CLI parsing with a real command model
- make configuration explicit
- introduce request/response objects for use cases

Rust focus:

- value objects
- enums and command modeling
- error propagation
- test structure

## Phase 2: Domain Expansion

Goal:
Grow the product from "today's schedule" into a useful scheduling engine.

Primary work:

- next-prayer queries
- schedule-for-date and schedule-for-range use cases
- timezone-aware outputs and validation
- explicit calculation profiles

Rust focus:

- domain-driven types
- ownership across request flows
- iterators and collection shaping
- serialization boundaries

## Phase 3: Application Boundaries

Goal:
Separate policy from infrastructure so the app becomes testable and extensible.

Primary work:

- ports for time, calculation, and configuration
- application services/use cases
- adapter boundaries for CLI and output
- test doubles where appropriate

Rust focus:

- traits as capability boundaries
- dependency injection without a framework
- borrowing vs owning across layers
- error taxonomy

## Phase 4: Production Readiness

Goal:
Make the system look like software that could survive real usage.

Primary work:

- config files and profile loading
- export adapters such as JSON and ICS
- logging and observability design
- benchmark or cost-model conversations
- packaging and release workflow

Rust focus:

- serde and external format boundaries
- module organization
- feature evaluation
- performance reasoning

## Phase 5: Senior Judgment

Goal:
Force deliberate engineering decisions instead of cargo-cult "best practice."

Primary work:

- evaluate threads for batch generation
- evaluate async and reject or adopt it with evidence
- perform architecture reviews from git history
- defend tradeoffs in writing

Rust focus:

- sync concurrency primitives
- async cost/benefit analysis
- API design
- architectural restraint

## Graduation Standard

The learner finishes when they can:

- ship meaningful Rust changes with minimal hand-holding
- explain what each type protects
- keep architecture coherent under change
- use Rust features because they fit the problem, not because they are available
- defend production concerns beyond syntax, including testing, packaging, and operational shape
