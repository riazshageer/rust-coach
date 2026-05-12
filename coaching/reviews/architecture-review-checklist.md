# Architecture Review Checklist

- Is each module responsible for one kind of concern?
- Are domain types insulated from third-party crate details where appropriate?
- Does the application layer orchestrate rather than perform all work itself?
- Are service boundaries real or accidental?
- Is the formatting layer presentation-only?
- Are configuration and runtime inputs modeled explicitly?
- Are error types structured around actual failure sources?
- Is the code easy to change in one area without rippling through everything?
- Did I add an abstraction because it is needed now, not because it might be useful later?
- Can I explain the architecture in a few sentences without hand-waving?
