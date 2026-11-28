# 0060. Checks answer in bounded time, and say whether they are complete

- Status: Proposed
- Date: 2026-11-27

## Context

Fraud checks, admission checks and safety checks can take unbounded time when
a system is loaded or attacked. Fraud-detection systems in production meet
this by promising an answer within a fixed time, even when that answer is
incomplete for lack of resources. Eric worked on such a system, with a
two-second guarantee.

A late perfect answer can be worse than a timely partial one. An incomplete
answer presented as complete is worse than both.

## Decision

Proposed:

**Every check has a stated time and work budget.** On expiry, the service
returns a bounded incomplete result rather than waiting indefinitely.
This is a measurable service objective while reachable, not a promise of a
response through a failed network or crashed server (0062). Cancellation
must also bound continuing background work (0050).

**Coverage and outcome are separate.** An answer states whether the required
checks completed, and separately reports allow, deny or indeterminate.
Complete means the declared checks ran, not that an inference is true or
that permission was granted. A partial answer identifies unchecked
categories to authorized reviewers; public replies must not expose
protected facts or sensitive detection rules.

**Partial is never presented as clean.** What a partial answer may allow
depends on the stakes: low-risk actions may proceed, while anything that
would disclose protected information, act for someone, or admit evidence
requires completion and a favourable result for the checks mandatory for
that action, or it waits or is refused (0055: confidentiality fails closed).
Policies identify those mandatory checks in advance; optional advisory
analysis must not become an indefinite veto.

**Permission is checked at the effect.** A result is scoped to the operation,
principal, resource version, policy version and validity window. Changed
permissions, expiry or revocation require revalidation before a read,
delivery, admission or external effect. A completed check is not a reusable
blanket capability.

**Pending is not admitted.** A delayed action receives an explicit pending
or refused status, safe retry and duplicate-handling semantics, and an
expiry or review path. A filing deadline is not suspended by a pending
check; the person is told the status and any available alternative (0062).

**Partial results and their disposition are recorded** in the flight
recorder under its audit and diagnostic policies (0059).

## Consequences

Overload changes how quickly things happen, never the truthfulness of what
Wave says about them (0050).

Before production, test budget exhaustion and cancellation, a complete
denial, incomplete mandatory checks, revocation between check and effect,
lost replies, duplicate retries and expiry of pending actions.
