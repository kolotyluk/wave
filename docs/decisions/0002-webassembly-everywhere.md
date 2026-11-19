# 0002. WebAssembly everywhere

- Status: Accepted
- Date: 2026-11-16

## Context

Wave runs in several places: on servers, in browsers, and as robots and doors
written by other people. Each place has its own way of running code.

## Decision

**If you don't have WebAssembly, you don't have Wave.** Every part of Wave
that runs, runs as WebAssembly: the server, the doors, the robots and the
code in the browser.

## Consequences

One sandboxed, portable kind of artifact everywhere. What is running can be
identified exactly.

Everything depends on WebAssembly runtimes being able to do what Wave needs,
networking above all. That is the first thing to prove (see 0026).
