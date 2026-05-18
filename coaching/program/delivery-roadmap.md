# Delivery Roadmap

This roadmap is the main execution plan for the repo.

## Milestone 1: Foundation Hardening

Brief:
Move the app from demo code to a disciplined baseline.

Required outcomes:

- real tests for domain validation and schedule behavior
- a command model instead of ad-hoc `std::env::args`
- explicit request objects for current behavior
- first pass at error boundary cleanup

Reference:
- `milestones/01-foundation-hardening.md`

## Milestone 2: Domain And Request Expansion

Brief:
Support real queries beyond today's hard-coded schedule.

Required outcomes:

- schedule-for-date
- schedule-for-range
- next-prayer query
- explicit calculation profile handling

Reference:
- `milestones/02-domain-and-request-expansion.md`

## Milestone 3: Application Boundaries

Brief:
Separate orchestration from infrastructure and make use cases independently testable.

Required outcomes:

- application layer introduction
- ports for clock and calculator behavior where justified
- response models that decouple interfaces from domain internals
- tests that prove boundary value

Reference:
- `milestones/03-application-boundaries.md`

## Milestone 4: Configuration And Adapters

Brief:
Add production-shaped external boundaries.

Required outcomes:

- config file support
- named profiles
- output adapter expansion
- CLI usability improvements

Reference:
- `milestones/04-configuration-and-adapters.md`

## Milestone 5: Quality And Operability

Brief:
Bring the repo closer to something a professional team would trust.

Required outcomes:

- stronger test strategy
- logging or observability design
- better error reporting
- packaging and release notes

Reference:
- `milestones/05-quality-and-operability.md`

## Milestone 6: Senior Engineering Judgment

Brief:
Use the repo to practice hard decisions, not just implementation mechanics.

Required outcomes:

- thread-or-no-thread decision with evidence
- async-or-no-async decision with evidence
- architecture review from git history
- written design defense

Reference:
- `milestones/06-senior-engineering-judgment.md`

## Active Starting Point

The current codebase should start at Milestone 1, tasking the learner with:

1. adding tests around `Location`
2. introducing a real CLI command enum
3. replacing hard-coded output selection with explicit command parsing

That is the first concrete coding slice because it produces real code, immediate review material, and better architecture pressure than another discussion prompt.
