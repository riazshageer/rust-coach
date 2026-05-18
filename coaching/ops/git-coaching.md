# Git Coaching Workflow

Git is part of the learning system, not just a storage tool.

## What To Use Git For

- seeing how your design evolved
- spotting repeated mistakes
- reviewing commit boundaries
- comparing first attempts with cleaner revisions

## Commit Guidance

- make one concept change per commit
- use commit messages that describe the design move
- avoid mixing refactors and feature additions when possible

## Questions The Coach Should Ask From Git History

- Did this commit preserve or strengthen an invariant?
- Did this diff make ownership clearer or noisier?
- Is a new abstraction justified by the change, or speculative?
- Did the tests move with the design?
- Does the commit message reflect the real architectural move?

## Useful Commands

```bash
git status --short
git log --oneline --decorate -8
git diff
git show --stat HEAD
```
