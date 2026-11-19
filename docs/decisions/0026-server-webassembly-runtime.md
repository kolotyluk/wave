# 0026. Which WebAssembly runtime hosts the server

- Status: Proposed
- Date: 2026-11-16

## Context

Everything assumes the server can run as WebAssembly (0002, 0006).

## Decision

Open. The first item in the order of work is a proof that `wave-server` runs as
WebAssembly in a container and accepts a connection, over HTTP/2 and, if
possible, HTTP/3.

## Consequences

If the proof fails, 0002 and 0006 are revisited before anything else is built.
