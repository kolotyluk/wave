# 0022. The record is evidentiary

- Status: Accepted
- Date: 2026-11-19

## Context

Waves will hold matters that end up in court: housing disputes first, justice
processes later. A record that cannot withstand scrutiny there fails the
people who relied on it.

In Canada, an electronic document must be authenticated (Canada Evidence Act,
s. 31.1), and the best evidence rule is satisfied on proof of the integrity of
the electronic documents system in which it was recorded or stored
(s. 31.2(1)(a)). Evidence of standards and practices may be presented on
admissibility (s. 31.5); the National Standard *Electronic Records as
Documentary Evidence* (CAN/CGSB-72.34) is the one usually cited. Provincial
Evidence Acts have equivalent provisions.

Courts ordinarily accept system integrity on testimony about how a system was
run. Some matters deserve more than that.

## Decision

A Wave's record is designed to be evidence: able to withstand scrutiny by a
court of law, and in some cases to meet a stronger standard.

The stronger standard is this: **a hostile expert, given an exported record
and Wave's open-source verifier, can confirm or refute its integrity without
trusting or contacting the operator.** Integrity is demonstrated, not
asserted.

To meet it:

- **Every operation is signed** by its author, and by the person a robot acts
  for, and is chained by hash to the history before it.
- **Nothing is altered.** A correction is a new operation. A redaction leaves a
  marker and the proof that something was there.
- **Time comes from outside.** The record is time-stamped at checkpoints by
  independent authorities (RFC 3161), not only by the server's clock.
- **History is witnessed.** Checkpoints of the record are published or held by
  parties independent of the operator, so history cannot be rewritten without
  detection.
- **Export is self-proving.** A Wave exports as a self-contained package that
  can be verified offline.
- **What ran is known.** Builds are reproducible, and the version of the server
  that admitted each operation is recorded.

## Consequences

This goal shapes the signed-operation envelope (0029), storage, export and
deployment, and it adds weight to the choice of a CRDT library that keeps
every change (0025).

It interacts with privacy law: lawful erasure must coexist with a record that
proves its own integrity (see 0029).

The framing in the Evidence Acts and CAN/CGSB-72.34 (including which edition
is current) is to be reviewed by counsel before the paper or any deployment
relies on it.
