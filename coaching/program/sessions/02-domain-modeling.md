# Session 02: Domain Modeling

## Outcome

Learn to spot when the code is carrying domain meaning in raw primitives instead of explicit types.

## What This Session Is Really Teaching

Enterprise Rust starts with the right nouns. If the domain is weakly modeled, everything built on top of it gets noisier and less safe.

## Source Focus

- `src/domain/location.rs`
- `src/domain/prayer_time.rs`

## Questions To Answer

- Which values in this app should never be "just a number"?
- Which types currently protect a real rule?
- Where is meaning still being smuggled through raw data?

## Work To Attempt

1. Identify one weak domain concept.
2. Write down the invariant you think it should enforce.
3. Ask the coach whether that invariant belongs in the type, constructor, or calling code.

## Exit Criteria

- you can explain why a type exists
- you can distinguish domain meaning from library convenience
- you can name at least one weakly modeled concept in the current app
