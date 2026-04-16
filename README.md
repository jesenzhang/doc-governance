# doc-governance

AI project documentation governance and OpenSpec-aware artifact generator.

## Positioning

`doc-governance` is a Rust CLI that scaffolds and validates an AI-first documentation system for software projects.
It combines:

- project-wide documentation governance
- OpenSpec-compatible change artifacts
- deployable outputs for coding agents such as Codex, Claude, OpenCode, and Kilo

## Design goals

- single Rust binary
- rtk-style lightweight CLI foundation
- generate agent-ready artifacts, not just source templates
- keep long-lived specs separate from per-change artifacts
- support multi-agent deployment targets

## Planned capabilities

- `doc-governance init` — initialize project governance and OpenSpec layout
- `doc-governance new change <name>` — create a new change workspace
- `doc-governance render --target <agent>` — render final deployable artifacts for an agent
- `doc-governance validate` — validate routing, policy, and spec consistency

## Initial architecture

- `src/cli.rs` — clap command definitions
- `src/main.rs` — CLI entrypoint
- `src/app.rs` — command dispatcher
- `src/scaffold.rs` — project scaffold generation
- `src/render.rs` — target-specific artifact rendering
- `src/validate.rs` — consistency checks
- `templates/` — embedded starter templates

## Near-term roadmap

1. bootstrap CLI and project initialization
2. add OpenSpec-compatible project skeleton generation
3. add target renderers for Codex / Claude / OpenCode / Kilo
4. add validation engine for documentation routing and update policy
