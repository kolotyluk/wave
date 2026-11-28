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

**Every check has a stated time budget**, and answers within it.

**Every answer says whether it is complete or partial.** A partial answer
names what was not checked.

**Partial is never presented as clean.** What a partial answer may allow
depends on the stakes: low-risk actions may proceed, while anything that
would disclose protected information, act for someone, or admit evidence
waits for a complete answer or is refused (0055: confidentiality fails
closed).

**Partial results are recorded** in the flight recorder (0059).

## Consequences

Overload changes how quickly things happen, never the truthfulness of what
Wave says about them (0050).
