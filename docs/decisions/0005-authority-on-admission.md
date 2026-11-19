# 0005. The server is the authority on admission, not on order

- Status: Accepted
- Date: 2026-11-16

## Context

With CRDTs no one orders operations. Something must still decide what enters
the record.

## Decision

The server is the authority on **admission**. It checks each operation's
signature and permission before relaying or storing it, and keeps the
append-only record. It does not decide the order.

## Consequences

Accountability has a place: a host admitted this operation, under these
rules, at this time.

Open for federation (step 7 of the order of work): two servers may decide
admission differently, and their copies would then differ for good. A likely
answer, inherited from Apache Wave, is that each Wave has a home server whose
admission is final. Also open: what happens to an operation that depends on
one that was refused. Both shape the protocol and are to be settled before
federation code is written.
