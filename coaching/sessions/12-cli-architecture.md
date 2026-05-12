# Session 12: CLI Architecture

## Learning Objectives

- evolve `main` into a clear command boundary
- parse user intent without overengineering
- know when a CLI parsing crate is justified

## Rust Philosophy Discussion

Command-line code is often the first place where architecture gets sloppy. Rust favors small explicit types for commands and flags over giant branching startup functions.

## Practical Code Walkthroughs

- `src/main.rs`
- `src/app/prayer_app.rs`

## Refactoring Exercises

- model current CLI behavior as an enum
- decide where command parsing should live if subcommands are added

## Architectural Reasoning

Today, `OutputFormat` is enough. Tomorrow, if commands grow, a command enum or small parser module becomes reasonable. The point is to scale only when the pressure is real.

## Ownership Analysis

CLI args are owned strings from the environment, but parsing should quickly turn them into stronger application input values so the rest of the app can work with typed intent.

## Common Beginner Mistakes

- letting `main` become the entire application
- silently ignoring invalid flags
- introducing a framework too early

## OOP-Thinking Traps

- command handler classes
- command registries before there are real commands

## Idiomatic Rust Alternatives

- small enums for command intent
- focused parsing functions
- errors that point to invalid user input clearly

## Challenge Exercises

- add a typed invalid-flag error
- add subcommands only if they improve the current user story

## Suggested Follow-Up Prompts For Codex

- “Review the CLI architecture and show me the smallest next step that keeps startup code explicit.”

## Self-Review Checklist

- Is `main` still readable?
- Are user inputs validated early?
- Did I resist building a command framework too soon?

## How An Expert Rust Developer Thinks About This

An expert keeps the entry point simple until it earns more structure, then introduces exactly one level of organization at a time.

## Suggested Refactors To The Real App

- make invalid flag handling explicit
- extract a small CLI input parser if another flag or subcommand is added
