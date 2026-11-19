# Wave

A walking skeleton for **Canada Wave**: public collaboration infrastructure in
which people, institutions and accountable software agents work on the same
evolving object. One record, complete history, transparent rules.

The idea is set out in the discussion paper *Canada Wave: A Public
Collaboration Infrastructure for Peace, Order, and Good Government* by Eric
Kolotyluk. This repository is where a small part of it becomes running code.

## Status

**Nothing is built yet.** This repository holds the plan and the decisions
made so far. Code follows.

**This repository is the source of truth.** Where it and the discussion paper
disagree, the repository governs. Until the repository is finalized,
integration runs both ways: the paper informs the repository, and decisions
made here are carried back into the paper. The two may be out of step in the
meantime.

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

## Principles

- **If you don't have WebAssembly, you don't have Wave.**
- **Invent only the model; standardize everything else.**
- **No back door.** Robots and people enter by the same interface.
- **No passwords, ever.**
- **The record is evidence.** It can withstand scrutiny by a court of law,
  and in some cases a stronger standard.

## Documentation

- [Architecture](docs/architecture.md): the system as it stands.
- [Decision records](docs/decisions/README.md): why it is that way, and what is
  still open for debate.
- [Test vectors](vectors/README.md): the language-neutral statement of the
  protocol.

## Order of work

1. A proof that `wave-server` can run as WebAssembly in a container and accept a connection.
2. `wave-core` and playback of a document's history.
3. The minimal Side Door, so the record can be inspected by eye from here on.
4. A scripted robot, to prove the participant interface before any model is involved.
5. `wave-door`: the door interface and the default Front Door.
6. A model behind a robot.
7. Two servers exchanging one Wave, to test federation.

## Who does what

- **Eric Kolotyluk** decides and verifies.
- **Claude** designs, writes, tests and reviews code. It works in this
  repository's folder and has no access to the servers.
- **Oz**, in Warp, builds and deploys to the alpha and beta stages.

Two tools with different permissions and a person between them: a small
working example of the separation of functions the paper argues for.

## Licence

Apache License, Version 2.0. See [LICENSE](LICENSE).

Copyright 2026 Eric Kolotyluk.
