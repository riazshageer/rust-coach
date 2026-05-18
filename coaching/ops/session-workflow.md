# Session Workflow

## Before You Start

1. Run `scripts/coach_context.sh`.
2. Start the agent with `coaching/codex/session-kickoff-prompt.md`.
3. Let the coach identify the active milestone, current task, and acceptance criteria.
4. Confirm only what is still ambiguous enough to block coding.

The learner should not have to manually reconstruct the plan.

## During The Work Block

- code first
- keep scope to one backlog slice
- ask for review after a meaningful diff, not after every thought
- use tests or compiler feedback to tighten the next step
- split work when a change mixes feature behavior, refactor, and cleanup

## Before You Stop

1. Review the current diff or final commit.
2. Update `coaching/state/current-session.md` if the official milestone changed.
3. Update local memory in `coaching/state/local/` with:
   - what changed
   - what is still open
   - the exact next restart step
4. Add a tracked session log only when a milestone closed or a durable lesson was learned.

## Completion Standard

A backlog slice is complete when:

- the code compiles
- behavior is covered by the right level of tests when practical
- the change fits the target architecture
- the diff can be explained clearly
- the next slice is obvious
