# 0054. Disputes are recorded and resolved, never overwritten

- Status: Proposed
- Date: 2026-11-24

## Context

Wave must always be prepared for fraud and confusion. People will say "that
wasn't me", "I never authorized that delegate", "these two accounts are the
same person", "this was altered". In the justice system above all, being able
to show who said what, and what was disputed and how it was decided, is
essential.

A record that quietly corrects itself cannot be trusted. A court needs not a
clean record but an honest one.

## Decision

Proposed: **a dispute is a first-class part of the record.**

- **A dispute is a claim.** Anyone affected may raise one, about an operation,
  an identity link, a delegation, an agreement, a bound or a moderation flag.
  It names what is disputed, by whom, and on what grounds, with evidence.
- **It has a status**, recorded at each change: open, under review, resolved
  (with the finding and who made it, under what authority), or withdrawn.
- **Nothing is overwritten.** What was disputed stays in the record. A
  resolution adds to it: a correction, a revocation, an unmerged identity, a
  reversed delegation. A corrected mistake stays visible as a corrected
  mistake.
- **Robots never resolve disputes.** They may gather evidence and flag; a
  person with the authority to decide does (0039, 0048).
- **Due process applies.** The parties are told, may respond, and may appeal,
  as for any adverse action (0047).
- **Disputes respect bounds.** A dispute about content inside a bound is
  raised and decided within that bound's rules (0036, 0037).

## Consequences

Disputes are themselves evidence: a court can see what was challenged, when,
by whom, and how it was resolved.

In code, dispute status and alias state are natural Rust enums, which make
illegal states unrepresentable: a dispute cannot be resolved without a
finding and a decision-maker, and an alias cannot be linked without proof
(0053).
