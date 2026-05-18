# Coaching State

This folder contains the durable coaching state for the repository.

## Two Layers

### Tracked Baseline

These files are checked in:

- `learner-profile.md`
- `course-progress.md`
- `current-session.md`
- `decision-log.md`
- `git-notes.md`
- `session-logs/`

Use them for durable context the whole repo should understand.

### Local Agent Memory

`coaching/state/local/` is git-ignored and should be created automatically by `scripts/init_local_coaching_state.sh`.

Use it for:

- exact current task
- restart notes
- short-horizon blockers
- architecture observations

The local layer is the default operational memory. The learner should not have to keep this current manually.
