# 0038. Agreement is a capability of participants

- Status: Proposed
- Date: 2026-11-20

## Context

Without prejudice requires every participant, robots included, to agree.
Other things will too: bounds, procedural rules, recordings, terms.

A robot is not a legal person. But British Columbia's Electronic Transactions
Act provides that a contract may be formed by the interaction of an electronic
agent and an individual, or of electronic agents (s. 16).

## Decision

Proposed: **agreement** is a capability of every participant: the ability to
make, record and honour an agreement on behalf of a principal.

- **An agreement is a signed operation.** It names what is agreed to, by whom,
  and for whom.
- **A robot agrees for its principal.** Its agreement is a signed operation by
  the robot, carrying a delegation from the person or institution it acts for
  (0029). It binds the principal, not the robot.
- **Agreeing means accepting enforced constraints.** Where an agreement limits
  what a robot may do, Wave enforces the limit; the robot's promise is not
  relied on.
- **Agreements are features of the capability.** Consenting to WP (0037) is the
  first. Others follow the same form.

## Consequences

Wave can make the strongest possible record of who agreed, for whom, and under
what constraints. Whether a given agreement is legally binding is for counsel
and, eventually, the courts. Robot agreement in particular is largely
unexplored in law.
