# Session 07: Traits And Capabilities

## Outcome

Learn when a trait represents a real capability boundary and when it is just abstract noise.

## What This Session Is Really Teaching

In enterprise Rust, the hard part is not writing traits. It is knowing when a trait earns its place.

## Source Focus

- `src/app/prayer_app.rs`
- `src/formatting/mod.rs`
- `src/services/mod.rs`

## Work To Attempt

1. Identify one boundary that might justify a trait.
2. Identify one boundary that definitely does not.
3. Explain what capability the trait would express and who would benefit from it.

## Exit Criteria

- you can justify a trait in capability terms
- you can reject a trait that adds indirection without benefit
- you can distinguish a stable boundary from speculative flexibility
