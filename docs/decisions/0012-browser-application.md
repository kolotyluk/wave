# 0012. The browser application hosts the doors

- Status: Accepted
- Date: 2026-11-16

## Context

A person needs to work with the record either by conversation or by looking
at its structure directly, and testing needs to see both at once.

## Decision

`wave-app` is Rust compiled to WebAssembly. It holds no door logic of its
own: it hosts the doors and connects them to the server.

A toggle switches between **door mode** (the Front Door) and **raw mode** (the
Side Door) over the same Wave. A split screen shows both side by side.

## Consequences

What is asked through the Front Door can be watched taking effect in the
record.

The core library runs unchanged in the browser.
