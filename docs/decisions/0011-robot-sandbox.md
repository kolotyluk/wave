# 0011. Robots are isolated by the WebAssembly sandbox

- Status: Accepted
- Date: 2026-11-16

## Context

Robots may be written by anyone.

## Decision

Robots run in a WebAssembly sandbox with only the capabilities the Wave
grants.

## Consequences

A robot can do what it was permitted to do and nothing else.
