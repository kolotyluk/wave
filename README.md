# Wave

A walking skeleton for **Canada Wave**: public collaboration infrastructure in
which people, institutions and accountable software agents work on the same
evolving object. One record, complete history, transparent rules.

The idea is set out in the discussion paper *Canada Wave: A Public
Collaboration Infrastructure for Peace, Order, and Good Government* by Eric
Kolotyluk. This repository is where a small part of it becomes running code.

## Status

**Nothing is built yet.** As of 2026-11-16 this repository holds the plan and
the decisions made so far. Code follows.

## What the skeleton must show

Two things, one on top of the other.

1. **The shared object.** A document that several participants edit at once,
   with every change attributed, every change kept, and the whole history
   playable from the beginning.
2. **The AI layer.** Robots that take part in a Wave as named participants,
   and a browser application that lets a person work with the same record
   either by conversation or by looking at its structure directly.

The agent never hides the record it is working on. That is the point of the
project.

## Goals

**Quality** and **trustworthiness**, in that order of dependence: the record
can only be trusted if the code that keeps it is correct, and can be shown to
be correct.

In practice:

- Tests come before features. The core library is tested with randomised
  concurrent edits that must always converge.
- Every operation is signed by the participant who made it, human or robot.
- Nothing reaches `main` without the checks passing.
- What runs on a server corresponds to a specific commit.

## Decisions so far

One principle sits above the table: **if you don't have WebAssembly, you don't
have Wave.** Every part of Wave that runs, runs as WebAssembly: the server, the
doors, the robots and the code in the browser.

| Question | Decision | Why |
| --- | --- | --- |
| Language | Rust throughout | Memory safety in the code that enforces permissions and keeps the record. |
| How concurrent edits converge | Conflict-free replicated data types (CRDT) | Live character-level editing without writing operational transformation from scratch. |
| Where authority sits | The server is the authority on *admission*, not on *order* | With CRDTs no one decides the order. The server checks each operation's signature and permission before relaying or storing it, and keeps the append-only record. |
| Server | Runs as WebAssembly, hosted by a WebAssembly runtime inside a container | The same sandboxed, portable artifact runs wherever Wave runs, and what is running can be identified exactly. |
| Front Door | Its own component, `wave-door`, pluggable, with a very decent default | The door stands between the person and Wave, so that nobody has to learn Wave to use it. Others can supply a different door without touching the record. |
| Robots | Join through the same interface as people | If there is no other route in, "no back door" is a property of the architecture, not a promise. |
| Robot isolation | WebAssembly sandbox, with only the capabilities the Wave grants | A robot can do what it was permitted to do and nothing else. |
| Browser application | Rust compiled to WebAssembly | One language end to end, and the core library runs unchanged in the browser. |
| Apache Wave | Specification lineage and reference, not runtime | The model and protocol documents are the inheritance. The retired Java code is something to test against, not to run. |

Still open:

- **Which CRDT library.** The record needs complete, attributed history, which
  favours a library that keeps every change. To be confirmed against the
  current state of the Rust options before any code depends on one.
- **How the server's WebAssembly is hosted.** Which runtime, and how it handles
  network connections and concurrency. This is the first thing to prove,
  because everything else assumes it.
- **Where the Side Door lives.** The native view of structure and history may
  be a second door behind the same interface as the Front Door, or part of
  the browser application.

## Planned layout

A Cargo workspace. Names are provisional.

| Crate | Role |
| --- | --- |
| `wave-core` | The shared object: document, operations, history, replay, permissions. No input or output. Compiles natively and to WebAssembly. |
| `wave-server` | Admits, stores and relays operations. Keeps the append-only record. Compiles to WebAssembly. |
| `wave-door` | The Front Door. It buffers the person from Wave: plain language in, visible structure out. Defines the interface a door must meet, and ships the default door. |
| `wave-robot` | The participant interface for robots, and the sandbox that hosts them. Separate from the door: a robot is a participant in a Wave, a door is how a person reaches one. |
| `wave-app` | The browser application that hosts a door. |

## Order of work

1. A proof that `wave-server` can run as WebAssembly in a container and accept a connection.
2. `wave-core` and playback of a document's history.
3. A scripted robot, to prove the participant interface before any model is involved.
4. `wave-door`: the door interface and the default Front Door.
5. A model behind a robot.
6. Two servers exchanging one Wave, to test federation.

## Test environment

A TrueNAS server running containers and virtual machines. One server runs in a
container; a second runs in a virtual machine so that federation is tested
between two independent machines.

## Who does what

- **Eric Kolotyluk** decides and verifies.
- **Claude** designs, writes, tests and reviews code. It works in this
  repository's folder and has no access to the servers.
- **Oz**, in Warp, builds and deploys to the test environment.

Two tools with different permissions and a person between them: a small
working example of the separation of functions the paper argues for.

## Licence

Apache License, Version 2.0. See [LICENSE](LICENSE).

Copyright 2026 Eric Kolotyluk.
