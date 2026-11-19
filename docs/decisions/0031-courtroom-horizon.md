# 0031. The architecture contemplates replacing the courtroom

- Status: Accepted
- Date: 2026-11-19

## Context

A long-term goal of Wave is to replace the contemporary courtroom with a Wave.
Justice is the hardest, highest-assurance destination the discussion paper
names, and the one it says comes last.

The skeleton will not build a court. But choices made now can make one
possible later, or rule it out.

## Decision

Wave's architecture contemplates a court proceeding conducted as a Wave: from
filing, through evidence and hearing, to judgment and appeal. Nothing in the
design may preclude it.

The skeleton does not implement it. Each decision is checked against it.

## Consequences

What a court needs, and where the design must leave room:

- **Authority granted by law, not by the Wave.** A presiding participant whose
  rulings carry authority that no owner of the Wave conferred. The permission
  model must allow roles that come from outside it.
- **Humans decide.** A ruling is an operation signed by the person with
  authority to make it. Robots may assist; the record shows that no robot
  decided.
- **Compartments.** Sealed material, in-camera hearings, without-prejudice
  exchanges and publication bans: parts of one Wave visible to some
  participants and not others.
- **The open-court principle.** A public view of the proceeding, with lawful
  redactions, derived from the same record (0032).
- **Procedure as rules.** Filing order, service and deadlines enforced as
  admission rules, with the rules themselves versioned and cited in the record.
- **Testimony.** Live and recorded audio and video, attached with integrity
  proofs, from witnesses identified at high assurance.
- **Independence.** The court is the admission authority for its own
  proceedings. No party, and no operator that could be a party, decides what
  enters the record. This bears directly on admission under federation (0005).
- **Decades, not years.** Court records outlive cryptographic algorithms.
  Signatures and time-stamps must be renewable before their algorithms
  weaken, as long-term archive standards such as Evidence Record Syntax
  (RFC 4998) provide. The encoding must remain readable for the life of the
  record.
- **Appeal.** Another court reads the same record, not a copy assembled for it.

This raises the bar on the record as evidence (0022), on identity (0021) and
on the signed-operation envelope (0029).
