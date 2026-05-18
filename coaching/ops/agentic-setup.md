# Agentic Coaching Setup

This repository is optimized for a coach-first coding agent, with first-class support for OpenAI Codex and GitHub Copilot CLI.

## Design Goals

- keep the learner writing code most of the time
- make restart cost near zero
- reduce manual note-taking
- use git history and diffs as coaching evidence
- hold the learner to production-grade standards

## What This System Implements

### 1. Repo-Level Coaching Rules

`AGENTS.md` and `.github/copilot-instructions.md` tell the agent to rebuild context, identify the active milestone, and keep the learner moving toward the next implementation slice.

### 2. Two-Layer Memory

Tracked memory in `coaching/state/` holds durable program context.

Git-ignored memory in `coaching/state/local/` holds:

- current task
- restart notes
- architecture observations
- short-horizon session ledger

The local layer is the agent's responsibility.

### 3. Product-Led Curriculum

The coach does not start from a generic topic. The coach starts from:

- the product vision
- the target architecture
- the roadmap milestone
- the current diff

That is what keeps sessions practical.

### 4. Prompt Library

The prompts are helper prompts, not the program itself. They exist to start quickly, request reviews, recover from blockers, and close sessions cleanly.

### 5. Git-Aware Review

The coach must use:

- `git status --short`
- `git log --oneline --decorate -8`
- `git diff`
- commit boundaries when present

The goal is to review design evolution, not just file snapshots.

## Operating Model

1. The agent bootstraps local memory if needed.
2. The agent rebuilds context from repo docs, state, and git.
3. The agent identifies the current milestone and next coding slice.
4. The learner codes.
5. The agent reviews and adjusts the next slice.
6. The agent writes restart context into local memory before the session ends.

## Failure Modes To Avoid

- turning every session into architecture talk
- letting the learner manage the roadmap manually
- asking for reflections before the learner has shipped anything
- introducing layers or traits without concrete pressure
