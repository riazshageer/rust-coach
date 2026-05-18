# Rust Prayer Times Coaching Repository

This repository is a guided Rust practice environment built around a real command-line application.

The code in `src/` is the working project. The material in `coaching/` turns that project into a structured coaching course focused on writing production-quality Rust with sound architecture, good domain modeling, and disciplined tradeoff analysis.

## What You Will Use

- `src/`: the application you will evolve
- `coaching/program/sessions/`: the main lesson sequence
- `coaching/program/exercises/`: targeted implementation tasks
- `coaching/program/reviews/`: review checklists for self-review and coach review
- `coaching/codex/`: ready-to-use prompts for common coaching interactions
- `coaching/state/`: your coaching notes for this repo, blank until you start using them
- `scripts/coach_context.sh`: a quick snapshot of the repo and coaching state

## Recommended Workflow

1. Create your own branch.
2. Read `coaching/README.md`.
3. Open `coaching/state/current-session.md` and choose the first session you want to work on.
4. Start Codex with the startup prompt in `coaching/codex/session-kickoff-prompt.md`.
5. Make your changes yourself.
6. Use the review prompts to get feedback, hints, and architecture guidance.
7. Record what you learned in `coaching/state/` before you stop.

## How The Coaching System Works

The coaching material is designed to support real working sessions, not passive reading.

- Each session has a specific objective, source focus, coaching loop, and exit criteria.
- The local state files start blank and only reflect work you have actually done in this repo.
- Git history is part of the learning loop. Commits, diffs, and reversals give useful evidence about how your Rust thinking is changing over time.
- The prompts are written to keep Codex in coaching mode by default, with implementation help available only when you deliberately ask for it.

## Coaching State

Use these files consistently:

- `coaching/state/learner-profile.md`
- `coaching/state/course-progress.md`
- `coaching/state/current-session.md`
- `coaching/state/decision-log.md`
- `coaching/state/git-notes.md`
- `coaching/state/session-logs/`

Treat them as an empty working notebook for the course. They are lightweight on purpose. Add only the notes you actually want to keep.

## Suggested Session Rhythm

1. Read the session file in `coaching/program/sessions/`.
2. Tell Codex what you think the session is really about.
3. Attempt the task before asking for a solution.
4. Ask for hints, not answers, when possible.
5. Request a review of your diff.
6. Log what changed in your understanding.

## Git Use

Make small commits on your branch. Good commit boundaries make the coaching better because they show your design path instead of just the final state.

Suggested pattern:

1. attempt
2. review
3. revise
4. commit
5. reflect

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

- Course overview: [coaching/README.md](/home/rshageer/Playground/prayer_times/coaching/README.md:1)
- Program overview: [coaching/program/README.md](/home/rshageer/Playground/prayer_times/coaching/program/README.md:1)
- Learning path: [coaching/learning-path.md](/home/rshageer/Playground/prayer_times/coaching/learning-path.md:1)
- Session kickoff prompt: [coaching/codex/session-kickoff-prompt.md](/home/rshageer/Playground/prayer_times/coaching/codex/session-kickoff-prompt.md:1)
- Prompt index: [coaching/codex/README.md](/home/rshageer/Playground/prayer_times/coaching/codex/README.md:1)
- Current session tracker: [coaching/state/current-session.md](/home/rshageer/Playground/prayer_times/coaching/state/current-session.md:1)
