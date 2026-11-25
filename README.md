# Wave

A walking skeleton for **Canada Wave**: public collaboration infrastructure in
which people, institutions and accountable software agents work on the same
evolving object. One record, complete history, transparent rules.

The idea is set out in the discussion paper *Canada Wave: A Public
Collaboration Infrastructure for Peace, Order, and Good Government* by Eric
Kolotyluk. This repository is where a small part of it becomes running code.

## Attribution

Inspired by the [Google Wave](https://en.wikipedia.org/wiki/Google_Wave) project, and
the [Wave Federation Protocol](https://en.wikipedia.org/wiki/Wave_Federation_Protocol)
that was developed for it. The Wave Federation Protocol is now
maintained by the [Apache Wave project](https://en.wikipedia.org/wiki/Apache_Wave).
Apache Wave is now archived.

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

## Why the Front Door

Google Wave's model was sound, but its interface showed people the machinery
and left them to work out what it was for. Too many people could not.

The Front Door hides that complexity for simple tasks, and for anyone who does
not want to see it, at least not all the time. The Side Door keeps it one tab
away. The complexity is hidden for convenience, never for secrecy.

In 2009 nothing could stand between a person and the structure and translate.
Now an AI agent can. Wave was ahead of its time; now is a better time.

## What Wave is for

***Collaboration has always been a journey. Canada Wave keeps the record of
it.***

**Canada Wave is the road network.** Highways, local streets, cul-de-sacs and
trails, built to common rules, so that many kinds of journey can be made.

**A Wave is the vehicle.** One case, one consultation, one proceeding, moving
through that network to where it needs to go. Most people just drive: that is
the Front Door. A mechanic opens the hood: that is the Side Door.

**A Wave is also the record of the journey.** Like an aircraft's flight
recorder, it keeps an honest account of what happened, signed by everyone
who took part, so that disputes can be settled and the system can learn.

A journey suits a Wave when it has all four of these:

1. **Several parties with different interests**, often including institutions
   and agents.
2. **One shared thing that changes over time**: a case, a draft, a decision.
3. **It will matter later who did what**: an audit, an appeal, a court, or a
   disagreement about what was agreed.
4. **Some people need it simple, and others need control.**

Wave is not for casual chat, a document only you edit, or broadcasting to the
world.

**The first journey is the Automated Housing Advocate**: a person with a
housing problem, helped in plain language, with a complete record they keep.

## Principles

- ***Those among us who are the most vulnerable require the most
  consideration and protection.*** Wave protects people in proportion to the
  harm they would suffer if it failed them.
- **If you don't have WebAssembly, you don't have Wave.**
- **Invent only the model; standardize everything else.**
- **No back door.** Robots and people enter by the same interface.
- **No passwords, ever.**
- **The record is evidence.** It can withstand scrutiny by a court of law,
  and in some cases a stronger standard.
- **Prepared for fraud and confusion.** Who said what can always be shown.
  Disputes are recorded and resolved, never settled by overwriting: a
  corrected mistake stays visible as a corrected mistake.

## Documentation

The four books (Wave Rider, Wave Shape, Wave Meister and Wave Reference) are
planned ([0044](docs/decisions/0044-documentation.md)). Until then:

- [Architecture](docs/architecture.md): the system as it stands.
- [Decision records](docs/decisions/README.md): why it is that way, and what is
  still open for debate.
- [Data](docs/data.md): logs, blips, artifacts, and what PostgreSQL is for.
- [Environments](docs/environments.md): Ztuph Wave, the development and
  demonstration instance.
- [Robot governance](docs/robot-governance.md): how AI robots are governed.
- [Features](docs/features.md): configurations of capabilities.
- [Messaging](docs/messaging.md): how Wave is explained, at three levels.
- [Reviews](docs/reviews/): reviews by Oz and others, kept as written.
- [Paper sync](docs/paper-sync.md): what the discussion paper still needs
  from this repository.
- [Test vectors](vectors/README.md): the language-neutral statement of the
  protocol.

## Order of work

1. **An authenticated echo server**: `wave-server` running as WebAssembly in a container, over HTTP/3 and IPv6, accepting authentication by several methods (passkey, SSH key, BC Services Card) and returning an echo signed with its own server identity. The first test vector.
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
