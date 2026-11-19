# 0025. Which CRDT library

- Status: Proposed
- Date: 2026-11-16

## Context

The record needs complete, attributed history (0004, 0022). Libraries differ
in whether they keep every change.

## Decision

Open. To be chosen against the current state of the Rust options, favouring a
library that keeps every change, before any code depends on one.

## Consequences

The choice decides what an operation is on the wire, so it precedes the
endpoints (0023) and the first test vector.
