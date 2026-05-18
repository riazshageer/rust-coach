# Agentic Coaching Setup

This repository is optimized for OpenAI Codex acting as a coach-first partner.

## Design Goals

- keep the learner doing the implementation work
- preserve context across restarts when notes exist
- make startup cheap
- use git history as coaching evidence
- provide reusable prompts for common moments

## What Was Missing In The Original Structure

- no enforced coach-first operating model
- no persistent state beyond the chat thread
- no standard startup routine for rebuilding context
- no simple tracker linking the learner to the right starting file and exit criteria
- no lightweight system for using commits and diffs as learning signals

## What This Repo Now Implements

### 1. Repo-Level Agent Instructions

`AGENTS.md` tells Codex to coach by default, review before coding, and reconstruct context from local files and git state.

### 2. Persistent Coaching Memory

`coaching/state/` stores durable learning context:

- learner profile
- course progress
- active topic
- decision log
- git-derived notes
- session logs

### 3. Startup Context

`coaching/codex/session-kickoff-prompt.md` gives a standard opening prompt that tells Codex exactly what to read and how to respond.

### 4. Prompt Library

The prompt set covers the common coaching loop:

- start work
- ask for hints
- get unstuck
- request review
- run architecture checkpoint
- review ownership
- review iterators
- generate the next challenge
- close the work block

### 5. Git-Aware Coaching

The process explicitly uses:

- `git status --short`
- recent commit history
- the current diff
- commit boundaries

This lets the coach comment on design evolution instead of just the final file state.

## Operating Model

1. Learner chooses a starting topic or continues an existing one.
2. Learner updates `coaching/state/current-session.md`.
3. Learner starts Codex with the kickoff prompt.
4. Codex reads the memory files and repo state.
5. Learner implements.
6. Codex reviews, questions, and guides.
7. Learner logs progress locally before stopping.

## Why This Works Better

- It reduces prompt-writing overhead.
- It gives Codex stable context when the learner chooses to keep notes.
- It keeps the learner responsible for the code.
- It creates a clear bridge between lesson intent and repository evidence.

## Notes On Prompt Engineering

The prompts in this repo use a few practical rules:

- define the role and constraints clearly
- anchor every response to repository evidence
- ask for structured outputs
- encourage internal comparison of alternatives before answering
- avoid hidden chain-of-thought disclosure while still asking for concise rationale
- force the coach to prefer hints and reviews over direct implementation
