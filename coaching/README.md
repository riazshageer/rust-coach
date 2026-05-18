# Rust Coaching System

This coaching system is built for a learner who already knows how to program, has read the Rust Book, and now needs deliberate reps shipping real Rust code. The coach is not here to replace implementation effort. The coach is here to drive judgment, pace, architecture, and review quality while the coachee keeps coding.

## Core Position

The previous failure mode for many coaching setups is obvious: too much discussion, too much note-taking, and too little delivery. This repository is designed to avoid that.

The operating assumption is:

- the learner should spend most sessions writing code
- the coach should continuously narrow the next coding move
- state should live in local agent-managed memory, not in the learner's head
- every milestone should produce working software, tests, and review evidence

## What The Coach Must Do

- reconstruct context from repo files, local memory, and git
- choose or confirm the next concrete coding task
- force justification when design choices are weak
- review diffs findings-first
- update local memory after major moves so restarts are cheap
- connect Rust language features to real architecture decisions

## What The Learner Must Do

- write the production code unless implementation help is explicitly requested
- explain reasoning before or after changes, not instead of changes
- keep diffs small enough to review
- use tests, compiler feedback, and git history as learning material
- treat each milestone as a delivery target, not a reading assignment

## System Components

- `program/`: the product target and implementation roadmap
- `ops/`: the working rules for the coach and learner
- `codex/`: helper prompts for common interactions
- `state/`: baseline tracked state plus git-ignored local memory

## Memory Model

There are two layers of memory:

1. Checked-in baseline in `coaching/state/`
   Used for durable repo-wide context such as learner profile, progress shape, and the official current milestone.

2. Git-ignored working memory in `coaching/state/local/`
   Used for agent-managed short-horizon state such as the active task, current blockers, architecture observations, and next-session restart context.

The local layer is the default operational memory. The learner should not need to manually maintain it.

## Working Rule

If there is any doubt between "talk more" and "define the next code slice," choose the next code slice.

## Read Next

- [program/README.md](/home/rshageer/Playground/prayer_times/coaching/program/README.md:1)
- [ops/session-workflow.md](/home/rshageer/Playground/prayer_times/coaching/ops/session-workflow.md:1)
- [ops/agentic-setup.md](/home/rshageer/Playground/prayer_times/coaching/ops/agentic-setup.md:1)
