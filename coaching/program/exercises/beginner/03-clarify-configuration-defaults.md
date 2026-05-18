# Beginner Exercise 03: Clarify Configuration Defaults

## Target Skill

Design constructors and defaults so convenience does not erase meaning.

## Change Request

- Introduce a clearer strategy for default configuration values.
- Keep the default setup readable from `main`.
- Avoid turning configuration into an unstructured bag of values.

## What The Coach Should Press On

- Which defaults are safe enough to be implicit?
- Which choices are important enough to stay explicit?
- Does the resulting API guide the caller well?

## Success Criteria

- defaults are intentional
- the configuration API stays small and readable
- important choices are not hidden accidentally
