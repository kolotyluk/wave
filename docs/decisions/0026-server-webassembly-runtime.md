# 0026. Which WebAssembly runtime hosts the server

- Status: Proposed
- Date: 2026-11-16

## Context

Everything assumes the server can run as WebAssembly (0002, 0006).

## Decision

Open. The first item in the order of work is a proof that `wave-server` runs as
WebAssembly in a container and accepts a connection over HTTP/3 and IPv6.

The component handles requests through wasi-http; the runtime terminates
QUIC, TLS and HTTP/3 (0041). The proof shows that a runtime can do this, and
that switching it to HTTP/2 or IPv4 needs no change to the component.

## Consequences

If the proof fails, 0002 and 0006 are revisited before anything else is built.
