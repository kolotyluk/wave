# 0033. Default doors, replaceable shells, and a tabbed app

- Status: Accepted
- Date: 2026-11-19
- Supersedes: [0012](0012-browser-application.md)
- Amended by: [0040](0040-deployments-may-mandate-doors.md)

## Context

0007 and 0008 established a Front Door and a Side Door but did not say what
the defaults are. 0012 put both in one browser application behind a toggle,
with a split screen.

The doors should be clear in the architecture, and others must be able to
supply their own.

## Decision

**The default Front Door is the Wave-supplied AI agent.** A person speaks or
writes in plain language; the agent works on the record for them, in view.

**The default Side Door is a GUI over the raw record**: Waves, wavelets,
documents, participants, permissions, history and operations, shown directly.

**Both defaults are replaceable.** Any agency may supply its own shell for
either door, so long as it conforms to the protocol, the door interface (0009)
and the design principles. No door has a route into the record that another
door lacks.

**The default app is a tabbed shell.** `wave-app` hosts the doors as tabs: a
Front Door tab, a Side Door tab, and a split tab showing both over the same
Wave. Other tabs may be added; the shell holds no door logic of its own.

## Consequences

What is asked through the Front Door can be watched taking effect in the
record, as before.

A replacement shell is judged by conformance, not by who made it. The
conformance suite must therefore cover doors, not only servers.

Tabs make room for later views (a sentry, an evidence export, a public view)
without changing the shell.
