# 0037. Without prejudice is a kind of bound

- Status: Proposed
- Date: 2026-11-20

## Context

In Canada, settlement privilege protects communications made in a good-faith
attempt to settle: they cannot be used to prove liability. Protection comes
from the substance of the communication, not from a label. The privilege
belongs to the parties jointly, and one exception allows the communications
to prove that a settlement was reached and on what terms (*Sable Offshore
Energy Inc. v. Ameron International Corp.*, 2013 SCC 37).

The same need, content that participants may see but not use against one
another, arises beyond settlement.

Without prejudice is not secrecy. Everyone inside sees everything. What is
restricted is use.

## Decision

Proposed: **without prejudice (WP)** is a capability, built as a specific kind
of bound (0036).

- **Entering.** Each participant, person or robot, consents before seeing
  anything inside. A late joiner consents on arrival.
- **Inheritance.** Wavelets spawned inside a WP bound are WP. A non-WP child is
  created only with the agreement of all participants. That is how an outcome
  leaves.
- **Direction.** Information may flow in from outside. Nothing flows out: no
  person, robot or export carries WP content into a wavelet outside the bound.
  The mark travels with the content, and the restriction is enforced, not
  requested (information-flow control).
- **Leaving.** A participant may leave. Leaving ends participation, not the
  protection of what was said.
- **Recording.** Everything inside is recorded under 0022, so that an
  exception, such as proving a settlement was reached, can be proved.
- **Opening.** The bound opens only by the agreement of all parties or by a
  ruling of a participant with authority granted by law (0031), and only as
  far as that agreement or ruling says.
- **Existence.** Whether those outside can see that a WP bound exists, and who
  is in it, is a setting of the bound.
- **Robots.** A robot's consent is its principal's consent (0038). A robot can
  consent only if its model capability is bound too: content sent to a model
  outside Wave is an outflow.
- **Proposing.** A robot that recognises settlement language may propose WP.
  The proposal and every response are recorded. Declining does not mean the
  conversation is unprotected: Wave records what the parties claimed; a court
  decides what the law protects.

## Consequences

Features are configurations of this capability: settlement, mediation,
apology, candid review and others ([features](../features.md)).

Whether settlement privilege extends to what an opposing party's agent saw,
and whether a court treats a robot's consent as its principal's, is untested.
To be reviewed by counsel before the paper relies on it.
