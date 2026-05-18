# Product Vision

## Product

Build a production-grade prayer-times platform with a strong core library and a CLI-first delivery surface.

The product should eventually support:

- calculating schedules for a single day, arbitrary dates, and date ranges
- querying the next prayer relative to a point in time
- selecting calculation profiles explicitly
- loading location and preferences from configuration
- rendering multiple outputs such as terminal, JSON, and calendar export
- supporting batch generation and future adapter expansion

## Why This Is A Good Coaching Product

This domain is small enough to understand but rich enough to force real engineering choices:

- domain invariants matter
- time and timezone handling matter
- CLI and configuration boundaries matter
- output adapters matter
- performance and concurrency can be evaluated honestly instead of theatrically

## Enterprise Standard For This Repo

"Enterprise grade" here does not mean framework-heavy. It means:

- explicit contracts
- coherent module boundaries
- testable use cases
- stable error handling
- low surprise APIs
- operational thinking
- change-friendly architecture

## Primary User Stories

1. As a CLI user, I want today's prayer schedule for a chosen location and profile.
2. As a user, I want the next prayer and time remaining.
3. As a user, I want schedule output for a date range.
4. As a user, I want reusable configuration profiles instead of repeating flags.
5. As an integrator, I want machine-readable outputs.
6. As a maintainer, I want architecture that can grow without becoming brittle.

## Non-Goals

- unnecessary distributed-systems complexity
- async for its own sake
- generic plugin systems before concrete need
- "clean architecture" diagrams without corresponding design pressure
