# Git Coaching Workflow

Git is part of the learning system, not just storage.

## What To Use Git For

- proving that the learner is shipping in small slices
- reviewing whether tests move with behavior
- spotting speculative abstraction early
- checking whether a refactor had a clear reason
- examining how design decisions evolved over time

## Commit Guidance

- keep one design move per commit when possible
- split refactor and behavior changes unless the split would be artificial
- use commit messages that describe the architectural move
- prefer a reviewable history over a clever history

## Questions The Coach Should Ask From Git History

- What did this diff make easier to change?
- What invariant became stronger or weaker?
- Did the code shape improve, or did the learner just make it compile?
- Was a trait, module, or abstraction introduced under real pressure?
- Would this change be understandable to another engineer six months later?

## Useful Commands

```bash
git status --short
git log --oneline --decorate -8
git diff
git show --stat HEAD
```
