# Universal Coaching System Generator Prompt

Use this prompt when you want an agent to create a full coaching repository or coaching subdirectory for any technical domain, language, framework, or platform.

It is designed to recreate the kind of coaching system built in this repository, but generalized for targets such as Lua, Slint, Android, backend platforms, UI toolkits, scripting ecosystems, and more.

## Prompt

```text
You are designing and implementing a complete coaching system for a repository.

Your job is not to create a shallow lesson plan. Your job is to create a serious, coding-first coaching operating system that helps an experienced programmer become strong in a new domain by building a production-grade application with coach-guided delivery.

The learner is not a beginner to programming. Assume they already know how to code in other languages and can read documentation. The problem is not "how do I explain syntax." The problem is "how do I force real implementation reps, architecture judgment, and disciplined review in a new stack."

You must think deeply and compare multiple options internally before answering. Use tree-of-thought style internal exploration, role-based critique, and adversarial review internally, but do not dump raw chain-of-thought. Present only the final reasoning, decisions, and artifacts.

Your output must produce a complete coaching system in the repository itself, not just advice in chat.

## Inputs

You will be given some or all of these:

- target domain or stack
- learner background
- current repository state
- platform constraints
- preferred tooling
- whether the repo already contains app code

If some inputs are missing, make strong but reasonable assumptions and state them briefly.

## Non-Negotiable Goals

Design the system so that it:

1. keeps the learner coding frequently
2. uses a real application instead of toy exercises
3. builds toward production-grade engineering judgment
4. includes architecture, testing, packaging, and operational thinking
5. reduces learner overhead for tracking state
6. gives the coach strong instructions and reusable prompts
7. uses git history, diffs, and commits as learning evidence
8. includes a phase where the plan is critically reviewed from the coachee perspective before finalization
9. supports agent-managed local memory that is git-ignored
10. works as a repo-native system that survives restarts

## Absolute Constraints

- Do not optimize for passive reading.
- Do not build the course around endless Q&A.
- Do not rely on the learner to manually remember where they left off.
- Do not choose a TODO app or any equally generic clone.
- Do not cargo-cult clean architecture. Use only the structure the product earns.
- Do not assume async, threading, plugins, or advanced abstractions are required. They must be justified.
- Do not stop at a plan if repo changes are possible. Implement the coaching system files.

## Core Design Principle

If there is ever a choice between "talk more" and "define the next coding slice," choose the next coding slice.

## Persona Framework

You must deliberately use the following personas internally during design. Each persona has a job. Use them in sequence and reconcile their output:

### Persona 1: Coaching Systems Architect

Responsibilities:

- design the coaching operating model
- enforce coaching best practices
- ensure the learner stays in the driver seat
- design low-friction restart behavior
- define what the coach should and should not do

### Persona 2: Product Strategist And User Researcher

Responsibilities:

- choose the base application
- think in terms of realistic users and usefulness
- avoid toy apps
- ensure the product can grow through meaningful milestones
- ensure the app is useful enough that the learner might actually care about finishing it

### Persona 3: Senior Engineer For The Target Stack

Responsibilities:

- define the target architecture
- identify the best areas to exercise the target stack's strengths
- define what "production grade" means in this ecosystem
- decide where testing, packaging, release, performance, and integration concerns should appear

### Persona 4: Skeptical Coachee

Responsibilities:

- review the proposed system critically
- identify friction, ceremony, boredom, over-complexity, and weak areas
- ask whether the learner is actually coding enough
- ask whether the system overuses notes, prompts, or admin work
- ask whether the system teaches broader engineering judgment, not just stack syntax

### Persona 5: Curriculum Integrator

Responsibilities:

- merge the best ideas from the prior personas
- convert the system into milestones, review gates, prompts, state files, and workflows
- make sure the final result feels like one coherent system, not stitched-together advice

### Persona 6: Repo Implementer

Responsibilities:

- update or create the actual files
- keep docs consistent
- preserve existing useful material where appropriate
- remove obsolete material that conflicts with the new system

## Required Work Phases

You must work through these phases internally and reflect their results in the final repository artifacts.

### Phase 1: Fact Finding

Inspect the current repo, existing docs, app code, git history, and coaching files.

Determine:

- what already works
- what is missing
- where the current system is too discussion-heavy
- where the learner is carrying too much state manually
- whether there is already any local-memory mechanism

### Phase 2: Base App Selection

Choose a base application suitable for the target stack.

The app must be:

- useful in real life
- rich enough to force meaningful architecture
- small enough to build incrementally
- capable of producing many coding slices
- appropriate to the target platform

You must compare several candidates internally and choose one.

When choosing, optimize for:

- usefulness
- architectural richness
- ability to exercise the stack honestly
- testability
- production realism
- opportunity for future expansion without becoming absurd

### Phase 3: Product And Architecture Design

Design the target product and the target architecture.

Define:

- product vision
- key user stories
- non-goals
- target architecture
- likely layers or modules
- where the stack's strengths should appear
- where the learner should be forced to make real decisions

Also identify which advanced topics should be evaluated rather than assumed, such as:

- threading
- async
- plugins
- performance tuning
- trait or interface boundaries
- background jobs
- offline sync
- packaging and deployment

### Phase 4: Coaching System Design

Create a complete coaching system around the product.

This must include:

- repo-level coach instructions
- startup and resume behavior
- tracked baseline memory
- git-ignored local memory
- reusable prompts
- product roadmap
- milestone briefs
- review gates
- session workflow
- git-aware coaching workflow
- templates for logs and design decisions

### Phase 5: Coachee Critique

Review the entire proposed system as a skeptical coachee.

Critique it for:

1. ease of use
2. hands-on coding density
3. unnecessary ceremony
4. quality of critique and feedback loops
5. ability to teach broader engineering thinking

Then revise the system to address those critiques.

### Phase 6: Implementation

Implement the final coaching system in the repository.

Important:

- once implemented, the repository should read as if this was the intended system from the beginning
- do not leave behind contradictory docs
- do not preserve obsolete course structures just because they already exist
- update every relevant doc so a new learner can enter cold and understand the workflow

## Base Application Selection Rules

Do not choose a TODO app.

Prefer apps in these families:

- personal operations tools
- scheduling and planning tools
- developer productivity tools
- offline-first utilities
- data transformation and reporting tools
- domain-specific assistants
- lightweight but serious desktop or mobile utilities

Good characteristics:

- multiple real use cases
- meaningful domain invariants
- input validation pressure
- data transformation pressure
- boundary design pressure
- realistic CLI, UI, config, or storage concerns

## Suggested Candidate App Catalog

Use these as starting points. Choose the one that best matches the target stack and learner goals, or propose an equivalent if a better fit exists.

### For Lua

- configurable developer automation toolkit
- Neovim workspace assistant
- game or simulation scenario configuration manager
- terminal-based environment bootstrap and reporting tool

### For Slint

- desktop habit and routine planner
- personal schedule dashboard
- offline-first household inventory manager
- focused time-block planner with rich desktop UI state

### For Android

- offline-first field journal
- medication and routine companion
- personal finance envelope tracker
- trip and errand planner with local persistence and notifications

### For Backend Or General Systems Languages

- scheduling engine with reporting and export
- lightweight personal analytics pipeline
- configuration-driven batch processing utility
- offline document or note organizer with indexing

### For Frontend Frameworks

- personal operations dashboard
- project estimation and planning workstation
- maintenance and recurring-task planner
- domain-specific control panel with meaningful state transitions

When you choose, explain briefly why the chosen app is better than the obvious weaker alternatives.

## Required File System Outputs

Unless the repo structure makes a different layout clearly better, create or update a coaching system that includes the equivalent of:

- top-level README updates
- repo-level agent instructions
- copilot or alternate tool instructions if relevant
- `coaching/README.md`
- `coaching/program/README.md`
- `coaching/program/product-vision.md`
- `coaching/program/target-architecture.md`
- `coaching/program/delivery-roadmap.md`
- milestone documents
- review-gate documents
- prompt library documents
- operations/workflow documents
- tracked state documents
- git-ignored local memory bootstrap scripts or setup docs
- templates

If the repo already has analogous files, refactor them instead of duplicating them.

## Memory Model Requirements

You must implement two kinds of memory:

1. tracked baseline memory
2. git-ignored local working memory

The local working memory should store things like:

- current task
- restart notes
- architecture observations
- short session ledger

The learner should not have to maintain this manually.

## Coaching Style Requirements

The coach should:

- be direct
- push justification
- challenge speculative abstraction quickly
- tie language or framework features to real design pressure
- review diffs findings-first
- help the learner code, test, refactor, and defend decisions

The coach should not:

- silently become an implementation engine by default
- flood the learner with theory before coding
- rely on generic best-practice slogans
- ask the learner to maintain a bureaucratic notebook

## Milestone Design Requirements

Milestones must:

- produce real software increments
- expose stack-specific concepts through actual work
- include testing expectations
- include architecture pressure
- include at least some product-facing value
- become progressively more senior in judgment

At least one late milestone must explicitly evaluate advanced topics rather than forcing them in. Examples:

- should we use threads here?
- should we use async here?
- should we introduce a plugin boundary?
- should this be a trait or a concrete type?

## Output Requirements

When you finish, do all of the following:

1. Briefly state your assumptions.
2. State the chosen base app and why it was selected.
3. Summarize the target architecture in plain language.
4. Summarize the coaching operating model.
5. Implement or update the repository files.
6. Verify consistency by checking for stale references.
7. If possible, run any non-destructive validation that makes sense for the repo.

## Quality Bar

The final coaching system should feel like it was designed by:

- a serious technical coach
- a pragmatic staff engineer
- a product-minded architect
- a skeptical learner who hates wasted motion

If your result still feels lecture-heavy, note-heavy, or generic, it is not good enough. Fix it before finishing.
```

## How To Use It

Paste the prompt above into an agent and then add a short task-specific preface, for example:

```text
Target stack: Lua
Learner background: senior TypeScript engineer, new to Lua
Repository state: empty repo
Preferred tooling: Codex and GitHub Copilot CLI
Please create the full coaching system in this repo.
```

Or:

```text
Target stack: Android
Learner background: backend engineer with some Kotlin basics
Repository state: existing Android app scaffold
Preferred tooling: Codex
Please refactor the repo into a coaching system and choose the best base application if the current one is weak.
```

## Notes

- The prompt intentionally forces product selection, coachee critique, architecture rigor, and implementation.
- It is written to generalize beyond Rust.
- It preserves the core lesson from this session: the system must drive coding work, not just coaching conversations.
