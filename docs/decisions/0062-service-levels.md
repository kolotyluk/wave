# 0062. Service levels by kind of action

- Status: Proposed
- Date: 2026-11-27

## Context

Card-fraud systems promise an answer within about two seconds, so as not to
keep the customer waiting and not to tip off a fraudster (0060). Wave is
different: a matter can run for months, and much of it waits on people. But
Wave is mission-critical (0055), and some of its actions cannot wait.

## Decision

Proposed: **service levels are set by kind of action**, not one promise for
everything.

- **Interactive.** The door responds, and an operation is acknowledged,
  within seconds. The receipt says which durability level it has reached
  (0043): accepted, durably stored, or witnessed. Later stages may follow;
  the receipt never overstates.
- **People in the loop.** Reviews, disputes and escalations (0054) carry
  deadlines known to the Wave, with notice before they fall due.
- **Danger.** Wave is not an emergency service and promises no emergency
  handling. But the referral itself is immediate: when someone states
  intent to harm, the agent's first reply is 9-8-8 or 911, ahead of any
  check (0061).
- **Recovery.** Each deployment states its recovery point (how much recent
  work may be lost) and recovery time (how quickly service returns).

**Checks a fraudster could watch do not reveal themselves.** Where a check's
timing or reply could be observed by the person being checked, it takes the
same time and looks the same whatever it finds. What it found goes to the
account holder through a channel they chose, not to whoever is at the
keyboard (0061).

**Outages are evidence.** Unavailability and overload are in the flight
recorder (0059), timed and witnessed, so a person who missed a legal
deadline because Wave was down can prove it.

**Objectives here, agreements in deployments.** The architecture sets
service-level objectives and makes them measurable. Service-level
agreements, with consequences, belong to each deployment.

## Consequences

Numbers wait for the echo server, which gives something to measure.
Recovery objectives drive the open questions on point-in-time backups and a
synchronous replica.
