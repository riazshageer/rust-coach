# Session 08: Module Organization

## Outcome

Use modules to reflect responsibility, not just folder count.

## What This Session Is Really Teaching

Module structure is an architectural statement. If the module tree is confused, responsibility is probably confused too.

## Source Focus

- `src/main.rs`
- `src/app/`
- `src/domain/`
- `src/formatting/`
- `src/services/`

## Work To Attempt

1. Explain what each top-level module owns.
2. Look for any file whose responsibility feels split or vague.
3. Decide whether the problem is naming, grouping, or API exposure.

## Exit Criteria

- you can explain the module tree in one pass
- you can spot one place where responsibility could be sharper
- you can avoid reorganizing files without a real design reason
