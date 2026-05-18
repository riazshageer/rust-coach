# Session 09: Type-Driven Design

## Outcome

Use the type system to encode decisions so the code becomes easier to use correctly.

## What This Session Is Really Teaching

Type-driven design is not about making clever types. It is about making the intended path obvious and invalid paths awkward.

## Source Focus

- `src/app/config.rs`
- `src/domain/`
- `src/errors/mod.rs`

## Work To Attempt

1. Find a place where a type could remove a conditional or comment.
2. Decide whether the gain is real or just theoretical.
3. Explain how the new shape would change the caller experience.

## Exit Criteria

- you can explain one place where the type system should do more work
- you can reject unnecessary type cleverness
- you can tie a type design back to a concrete maintenance benefit
