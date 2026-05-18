# Target Architecture

## Architectural Direction

The target shape is a layered but lightweight Rust application:

- `domain/`: pure domain types, invariants, and domain services
- `application/`: use cases, request/response models, capability ports
- `infrastructure/`: adapters to external crates, filesystem, clock, and exports
- `interfaces/cli/`: command parsing and CLI presentation

The current codebase is not there yet. That gap is intentional and becomes the delivery roadmap.

## Core Boundaries

### Domain

Owns:

- location and coordinate validation
- prayer schedule concepts
- request invariants
- calculation profile concepts
- domain terminology

Should not own:

- CLI parsing
- file I/O
- formatting concerns
- direct dependency on app wiring

### Application

Owns:

- use cases such as `get_today_schedule`, `get_schedule_for_date`, `get_next_prayer`
- request and response DTOs
- orchestration across ports
- application error mapping

Should not own:

- `std::env` parsing
- concrete serializer or filesystem choices
- direct time-source assumptions when that matters for tests

### Infrastructure

Owns:

- adapter to `salah`
- configuration file loading
- JSON and ICS export adapters
- system clock
- caching if introduced

Should not own:

- business rules that belong in the domain
- use-case policy

### Interfaces

Owns:

- CLI commands
- argument validation at the syntax boundary
- rendering decisions for the terminal experience

Should not own:

- business logic
- direct manipulation of domain internals beyond stable use-case contracts

## Design Principles

- model invalid states out of existence where practical
- keep traits at capability seams, not everywhere
- borrow data when it clarifies lifetime and ownership; own data when it simplifies use-case flow
- hide third-party crates behind meaningful local abstractions only when that abstraction buys testability or portability
- prefer one obvious path through the codebase over abstract flexibility

## Rust Features The Coach Should Deliberately Leverage

- newtypes and smart constructors
- enums for command and mode modeling
- traits for capability ports
- iterators for schedule transformation pipelines
- `Result` and error enums for boundary clarity
- pattern matching for control flow
- tests across unit, module, and integration levels
- sync concurrency for batch generation only if the workload justifies it
- async only after evidence says I/O and lifecycle complexity are worth it

## Deliberate Decision Prompts

The coach should explicitly ask these when relevant:

- Should this be a trait, or is a concrete type clearer?
- Does this request model belong in the domain or application layer?
- Is concurrency improving throughput here, or just increasing complexity?
- Do we need async, or is a synchronous boundary more honest?
- Is this abstraction reducing coupling, or hiding simple code?
