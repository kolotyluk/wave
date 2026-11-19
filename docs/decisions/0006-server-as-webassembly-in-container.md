# 0006. The server runs as WebAssembly in a container

- Status: Accepted
- Date: 2026-11-16

## Context

The server must run the same way on a developer's machine, on test hardware
and in a commercial cloud.

## Decision

The server compiles to WebAssembly and is hosted by a WebAssembly runtime
inside a container.

## Consequences

The same sandboxed artifact runs wherever Wave runs, and what is running can
be identified exactly.

Which runtime, and how it handles connections and concurrency, is open
(see 0026).
