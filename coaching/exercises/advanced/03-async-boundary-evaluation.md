# Advanced Exercise 03: Evaluate An Async Boundary

## Goal

Learn when async belongs and when it does not.

## Change Request

- Add one integration idea that could plausibly justify async, such as fetching remote location data or reading external configuration sources.
- Write an architecture note comparing sync and async versions before changing code.
- Only implement async if the use case truly benefits.

## Why This Matters

Rust async has real complexity costs. This exercise teaches disciplined adoption instead of trend-following.

## Self-Review

- Was async actually necessary?
- Did the boundary become clearer or just more complex?
- Could a synchronous design still be the better architecture here?
