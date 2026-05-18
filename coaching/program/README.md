# Program Overview

This directory is the source of truth for what gets built and how the coaching progresses.

The program is organized around product delivery, not abstract lessons.

## Source Of Truth

- `product-vision.md`: what the finished product should do and why it matters
- `target-architecture.md`: the architectural shape the coach is steering toward
- `delivery-roadmap.md`: the ordered implementation plan
- `milestones/`: detailed milestone briefs with required coding work
- `review-gates/`: the standards used to decide whether a slice is actually done

## How To Use This Program

1. Read the product vision and target architecture.
2. Open the roadmap and identify the active milestone.
3. Use `coaching/state/current-session.md` plus `coaching/state/local/` to confirm the exact task in flight.
4. Code the smallest slice that moves the milestone forward.
5. Review the diff against the relevant review gate.

If a discussion does not end in a clearer coding move, it is not serving the program.
