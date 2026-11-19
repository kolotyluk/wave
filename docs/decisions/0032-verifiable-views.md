# 0032. Views are verifiable projections of the record

- Status: Proposed
- Date: 2026-11-19

## Context

A courtroom is open to anyone unless the case is restricted. In Wave,
different credentials should allow different views of the same record: the
public, the parties, counsel and the court each see what they are entitled to.

Apache Wave controlled access per wavelet. A wavelet was seen whole or not at
all; there was no redaction within one.

A view that the server merely filters must be trusted, and so cannot be
verified. That fails the evidentiary goal (0022).

## Decision

Proposed: visibility works at two levels.

**Compartments are wavelets.** Sealed material, in-camera hearings and
without-prejudice exchanges each live in their own wavelet, with their own
participants, as in Apache Wave.

**Views within a wavelet are verifiable projections.** What a credential may
see is decided by the rules of the Wave (0021, 0031). What it may not see is
replaced by a commitment: a salted hash of the content. Signatures cover the
commitments, not the content, so every view verifies. A viewer can confirm
that something was there, who wrote it and when, and that nothing else was
changed, without learning what it was.

The salt prevents anyone from testing guessed content against a hash. The
same technique gives selective disclosure in ISO mobile driving licences.

The open court is the view granted to an anonymous credential. Restricting a
case narrows or withdraws that grant.

## Consequences

One decision serves several needs: redaction, lawful erasure (destroy the
content, keep the commitment), selective disclosure, and the public view of a
proceeding.

It fixes part of the signed-operation envelope (0029): signatures are over
commitments. It must be settled before the first test vector.

A projection must still converge: the CRDT works over commitments where
content is withheld (0025).

Views are derived from one record. Nobody receives a copy assembled for them.
