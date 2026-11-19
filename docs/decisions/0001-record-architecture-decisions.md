# 0001. Record architecture decisions

- Status: Accepted
- Date: 2026-11-19

## Context

Decisions were collecting in a table in the README. A table records what was
decided but not what it replaced, and it has no room for a decision that is
still being argued.

The repository is the source of truth for Wave. The discussion paper and the
repository inform each other until the repository is finalized.

## Decision

Each architecture decision is a numbered Markdown file in `docs/decisions/`,
in the MADR form: context, decision, consequences.

A record has a status: **Proposed** (open for debate), **Accepted**, or
**Superseded by** a later record. An accepted record is not rewritten. When a
decision changes, a new record supersedes it, and both remain.

`docs/architecture.md` describes the system as it stands and links here.

## Consequences

The design has an attributable, replayable history of its own, which is the
same property Wave promises for its records.

Drafts are visible as drafts. Anything Proposed is something to debate, not
something to build on.
