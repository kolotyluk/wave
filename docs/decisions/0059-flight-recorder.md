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
time-stamped and witnessed like the record (0022). This makes changes to
committed history detectable against independently held checkpoints; it
does not prove that every real-world event was captured, or protect an
unwitnessed tail against every compromised-host attack.

**Audit and diagnostics have different guarantees.** Protected-data access,
authority changes, break-glass use, lawful disclosure and safety decisions
are mandatory audit events, never sampled. High-volume diagnostics and
rejection details may be sampled or aggregated under a published policy,
with coverage and dropped-event counts stated. Admission receipts remain
in the authoritative log (0043), not a sampled substitute.

**Sensitive actions require a durable audit trail.** Before protected data
leaves the controlled boundary or an authority change takes effect, durable
audit intent must be recorded; outcome or uncertain completion is appended.
The action and intent are atomic where possible; external effects require
reconciliation, not a claim of atomicity across systems. If audit recording
fails, those actions fail closed. Recovery uses a separately protected,
durably audited host path, with reconciliation when service resumes (0051);
it is not an unaudited bypass.

**It minimizes data; it is not automatically non-sensitive.** No credentials,
keys, tokens, protected payloads or free-form copies of user input. Access
times, actor identifiers and relationships can themselves expose a sealed
matter or a pseudonym-root link. Use scoped identifiers and protected audit
compartments, with least-privilege access and controlled correlation
(0050, 0053). Readers see only the audit view they are authorized to see.

**It is bounded without silently losing audit evidence.** Separate retention,
legal holds, storage reservations and capacity alarms apply to mandatory
audit and sampled diagnostics. State the durability level reached and any
known gaps. Exhaustion sheds diagnostics or stops affected sensitive actions;
it never silently overwrites mandatory events still under retention.

**It is for learning.** Its first use is incident investigation in the TSB
spirit: what happened, why, and how to prevent it. Its access is itself
recorded. The TSB analogy is a purpose, not a grant of legal privilege or
immunity from lawful production.

## Consequences

Incident reconstruction is supported within the stated capture, retention,
durability and trust limits. Independent observations are needed for events
the server cannot record, including its own complete failure (0062).

Which fields are recorded, and who may read the flight recorder, must be
specified before production, along with failure, recovery, audit-export
verification and capacity tests.
