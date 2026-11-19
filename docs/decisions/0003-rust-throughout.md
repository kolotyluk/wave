# 0003. Rust throughout

- Status: Accepted
- Date: 2026-11-16

## Context

The code that enforces permissions and keeps the record is the code whose
failure would defeat the system.

## Decision

Wave is written in Rust, end to end, including the browser application.
`unsafe` code is forbidden across the workspace.

## Consequences

Memory safety where it matters most, and one language for every component.

Rust has no stable binary interface, so crates are divisions of the source,
built together in one build. What ships are WebAssembly modules.

The protocol itself carries no language requirement (see 0017).
