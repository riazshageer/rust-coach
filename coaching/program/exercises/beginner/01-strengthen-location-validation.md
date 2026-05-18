# Beginner Exercise 01: Strengthen Location Validation

## Target Skill

Encode real invariants in domain constructors.

## Change Request

- Add focused tests for valid and invalid coordinate construction.
- Decide explicitly whether NaN and infinite values are allowed.
- If they are not allowed, make the constructors reject them.

## What The Coach Should Press On

- Is the invariant complete or only partially enforced?
- Does the constructor stay readable?
- Are the tests documenting the rule clearly enough for another engineer?

## Success Criteria

- invalid coordinates fail at the right boundary
- tests cover the most important edge cases
- the API still reads like domain code, not validation plumbing
