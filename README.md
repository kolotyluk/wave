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
| Side Door | Built early, in a minimal form, as the way to inspect the record | It shows structure, permissions, agents and history directly, which is what manual and sanity testing need. It starts as a test instrument and grows into the Side Door the paper describes. A second door from the start also keeps the door interface honest. |
| Door interface | Independent of how the person communicates | The first door is a browser door. Voice, telephone and text-message doors come later, so the interface carries what the person means and what Wave answers, and never assumes a screen. |
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

Notes for the protocol:

- **Rust source is the working definition, for now.** The protocol lives in
  `wave-protocol` and nowhere else. Wave must be implementable in any
  language, so a written specification follows when a second implementation
  is attempted.
- **Encoding is exact.** Every operation is signed, and a signature is over
  exact bytes, so two implementations must produce the same bytes for the
  same operation. The encoding is specified deliberately and never left to a
  library's defaults.
- **Test vectors are the neutral truth.** The `vectors/` directory holds
  example messages with their expected bytes and results. They are not Rust.
  Any implementation must pass them, which makes them the start of the
  conformance suite.
- **The robot and door boundary gets an interface definition.** Robots and
  doors are WebAssembly components that others may write in other languages,
  so that boundary is to be described in WebAssembly's own interface
  language. The tooling is to be verified before anything depends on it.
- **Asking is separate from changing.** Anything that changes a Wave is a signed
  operation. A request that only asks can never alter the record.
- **Use HTTP QUERY for asking.** Wherever the protocol uses HTTP, a request
  that reads and needs a structured body (a range of history, a search, an
  authorised semantic query) uses the QUERY method, RFC 10008: safe,
  idempotent, cacheable and repeatable, and it keeps the query out of the
  address. Support in libraries, proxies and gateways is to be tested early.

Deferred, not forgotten:

- **The full Side Door.** The paper describes a second entrance for those who
  need direct control. The skeleton builds only the minimal form, for
  inspection and testing. Editing through it, and making it fit for the
  public, come later.
- **Voice, telephone and text-message doors.** Later, but accounted for in the
  door interface from the start.

## Planned layout

A Cargo workspace, built from source in one build. Crates live under
`crates/`. Names are provisional. Only `wave-protocol` exists so far.

Rust has no stable binary interface, so nothing here is shipped as a prebuilt
library. The crates are divisions of the source. What ships are WebAssembly
modules: one for the server, one for the browser, and one for each robot.

| Crate | Role |
| --- | --- |
| `wave-protocol` | The protocol: the messages and operations that servers, doors and robots exchange, and the rules for encoding them. Nothing else. No dependencies. |
| `wave-core` | The shared object: document, operations, history, replay, permissions. No input or output. It has no artifact of its own: it is source, compiled into every component that uses it, so the server and the browser run the same code. |
| `wave-server` | Admits, stores and relays operations. Keeps the append-only record. Compiles to WebAssembly. |
| `wave-door` | The Front Door. It buffers the person from Wave: plain language in, visible structure out. Defines the interface a door must meet, in terms that do not depend on a screen, and ships the default door. |
| `wave-side` | The minimal Side Door: a direct view of a Wave's structure, permissions, participants and history. Read-only at first. |
| `wave-robot` | The participant interface for robots, and the sandbox that hosts them. Separate from the door: a robot is a participant in a Wave, a door is how a person reaches one. |
| `wave-app` | The browser application. It holds no door logic of its own: it hosts the doors and connects them to the server. A toggle in the interface switches between **door mode** (the Front Door) and **raw mode** (the Side Door) over the same Wave. A split screen shows both side by side, so that what is asked through the Front Door can be watched taking effect in the record. |

## Order of work

1. A proof that `wave-server` can run as WebAssembly in a container and accept a connection.
2. `wave-core` and playback of a document's history.
3. The minimal Side Door, so the record can be inspected by eye from here on.
4. A scripted robot, to prove the participant interface before any model is involved.
5. `wave-door`: the door interface and the default Front Door.
6. A model behind a robot.
7. Two servers exchanging one Wave, to test federation.

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
