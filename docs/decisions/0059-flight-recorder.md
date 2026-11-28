# 0059. A flight recorder for the system

- Status: Proposed
- Date: 2026-11-27

## Context

Wave's record keeps what people and robots said and did. It does not keep
what the system itself did: which operations it refused and why, when it was
overloaded, which policies changed, who opened protected information, when
someone broke the glass.

After an aircraft accident, investigators read the flight recorders to learn
what happened. Canada's Transportation Safety Board investigates to learn,
not to blame. Wave needs the same capability for its own failures.

## Decision

Proposed: **every server keeps a flight recorder**: a tamper-evident record of
the system's own behaviour.

It records:

- admission decisions, including refusals and their reasons;
- errors, faults and overload, and what was shed or delayed;
- robot actions and interventions (0061);
- policy and Admin Wave changes (0051);
- access to protected information, and by whom;
- break-glass use (0051);
- checks that returned partial results (0060).

**It is evidence too.** Entries are chained and periodically checkpointed,
time-stamped and witnessed like the record (0022), so an operator cannot
quietly rewrite what the system did.

**It holds no secrets and no protected content.** It records that protected
information was opened, not what it said. No keys, tokens or credentials, and
nothing that would reveal a sealed matter or a pseudonym's link to its
person (0050, 0053).

**It is bounded.** It has a defined size, retention and sampling policy, and
counts against resource limits (0050).

**It is for learning.** Its first use is incident investigation in the TSB
spirit: what happened, why, and how to prevent it. Its access is itself
recorded.

## Consequences

Every incident in a mission-critical service (0055) can be reconstructed.

Which fields are recorded, and who may read the flight recorder, must be
specified before production.
