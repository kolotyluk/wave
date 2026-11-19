# 0004. Concurrent edits converge by CRDT

- Status: Accepted
- Date: 2026-11-16

## Context

Several participants edit one document at once. Apache Wave reconciled
concurrent edits by operational transformation (OT) under a server's
authority. OT is specified for Wave but is hard to implement correctly from
scratch. Conflict-free replicated data types (CRDTs) have matured since 2010.

## Decision

Concurrent edits converge by CRDT.

## Consequences

Live, character-level editing without writing OT from scratch.

No one decides the order of operations, so authority has to sit somewhere
else (see 0005).

The record must keep every change, attributed, which narrows the choice of
library (see 0025).

The discussion paper (Appendix G, v0.4) still proposes OT first. The
repository governs; the paper is to be brought into line.
