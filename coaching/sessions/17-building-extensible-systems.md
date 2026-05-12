# Session 17: Building Extensible Systems

## Learning Objectives

- extend the app without speculative abstraction
- choose between enums, traits, and concrete branching
- design for plausible growth paths

## Rust Philosophy Discussion

Extensibility in Rust is best when it follows actual variation. The language gives multiple tools for variation, and the architect’s job is to choose the lightest one that fits the pressure.

## Practical Code Walkthroughs

- formatter system in `src/formatting/`
- configuration in `src/app/config.rs`

## Refactoring Exercises

- list realistic extensions to this app for the next six months
- for each one, decide whether it wants a new type, enum, trait, or module

## Architectural Reasoning

Formatters are a good example of justified extensibility. The variation is real, the capability is clear, and callers benefit from depending on one formatting contract.

## Ownership Analysis

Extensible designs should not hide ownership. A clean extension point still makes it obvious who owns inputs, outputs, and long-lived state.

## Common Beginner Mistakes

- generalizing for every imagined feature
- using trait objects when enums or concrete types would be clearer
- baking extension points into every layer

## OOP-Thinking Traps

- plugin fantasies before there are two real use cases
- abstract base mentality through traits

## Idiomatic Rust Alternatives

- closed-set enums for known variants
- traits for open capability variation
- concrete types for single-path logic

## Challenge Exercises

- add one new formatter or command path
- justify the chosen abstraction explicitly in writing

## Suggested Follow-Up Prompts For Codex

- “Help me extend this repository, but reject any abstraction that is not justified by a real growth path.”

## Self-Review Checklist

- What variation am I supporting?
- Is the abstraction lighter than the future change it enables?
- Could a simpler design still work today?

## How An Expert Rust Developer Thinks About This

An expert designs for plausible change, not imaginary universes. They keep concrete code until variation becomes a real design pressure.

## Suggested Refactors To The Real App

- consider a config-loader boundary only when file support is added
- keep services concrete unless multiple implementations appear
