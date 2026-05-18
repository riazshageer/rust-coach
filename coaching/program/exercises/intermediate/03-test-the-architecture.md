# Intermediate Exercise 03: Test The Architecture

## Target Skill

Use tests to protect important boundaries and invariants.

## Change Request

- Add or improve tests that guard meaningful design decisions.
- Choose guarantees that would be expensive to weaken accidentally.
- Make the tests teach a future reader what matters.

## What The Coach Should Press On

- Does each test protect a real design property?
- Is the test too tied to irrelevant implementation detail?
- Would a failing test teach the next maintainer something useful?

## Success Criteria

- tests protect important behavior or boundaries
- failures would be informative
- the suite stays pragmatic
