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
  within a measured target of seconds while service is reachable. Pending,
  rejected and admitted are distinct states. For an admitted operation the
  receipt states the durability level reached (0043): committed on one
  server; committed and durably acknowledged by a synchronous replica; or
  sealed, time-stamped and independently witnessed. Later stages may follow;
  the receipt never overstates. A timeout is not proof of rejection: clients
  can reconcile operation status and retry without duplicate effects.
- **People in the loop.** Reviews, disputes and escalations (0054) carry
  deadlines known to the Wave, with notice before they fall due.
- **Danger.** Wave is not an emergency service and promises no emergency
  handling or monitoring. When available, the door promptly presents 9-8-8
  for suicide risk or 911 for imminent danger, without waiting for optional
  checks or model generation (0061). Static help remains accessible if the
  model is unavailable. A referral does not mean a human has been alerted;
  any actual escalation states its confirmed status.
- **Recovery.** Each deployment states its recovery point (how much recent
  work may be lost) and recovery time (how quickly service returns), per
  data class and failure scenario. These must meet the accepted requirement
  to preserve admitted evidence (0055); a nonzero recovery point cannot
  silently qualify that requirement. Any weaker development guarantee is
  stated explicitly and is unsuitable for production reliance.

**Checks minimize observable leakage.** Where a check's timing or reply could
be observed by the person being checked, it uses a consistent bounded
response envelope where practicable, with no unnecessary differences in
status, size, timing or retries that reveal sensitive findings.
Exact indistinguishability across networks and later outcomes is not
promised. Deployments threat-model and test observable side channels under
load; padding and delayed responses are bounded too (0050). Required safety
notices and recovery remain available. Findings go only to authorized
reviewers or the account holder through a previously verified safe channel;
being at the keyboard does not establish either authority (0061).

**Outage evidence has independent sources.** The flight recorder captures
what the server can observe (0059). External probes in independent failure
domains and privacy-preserving client submission receipts support failures
the server cannot log. Evidence states observation time, clock uncertainty,
coverage and gaps; no one source proves all users' reachability or that a
particular filing was attempted. Deployments publish incident attestations
and alternative filing routes. A court or other competent authority, not
Wave, decides any deadline relief.

**Objectives here, agreements in deployments.** The architecture sets
service-level objectives and makes them measurable. Service-level
agreements, with consequences, belong to each deployment. Targets define
the measurement point, observation window, percentile, load and failure
conditions. Human-review deadlines need staffing, escalation and overdue
notice; they are not software response guarantees.

## Consequences

Numbers wait for the echo server, which gives something to measure.
Recovery objectives drive the open questions on point-in-time backups and a
synchronous replica.

Recovery drills verify admitted-evidence preservation, not merely service
restart. Failure tests include server and probe outages, model unavailability,
clock uncertainty and lost acknowledgements; capacity tests measure tail
latency and side-channel leakage as well as average response time.
