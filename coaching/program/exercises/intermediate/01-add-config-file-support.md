# Intermediate Exercise 01: Add Config File Support

## Target Skill

Translate external data into trusted domain values without leaking weak shapes inward.

## Change Request

- Add support for reading app configuration from a file.
- Decide where deserialization should happen.
- Decide where conversion into domain-safe types should happen.

## What The Coach Should Press On

- Are external structs staying at the boundary?
- Is validation happening before trusted values enter the core?
- Is the happy path still readable?

## Success Criteria

- raw file data is separated from trusted config
- the translation path is easy to explain
- errors remain meaningful at the boundary
