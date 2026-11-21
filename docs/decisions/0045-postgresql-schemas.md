# 0045. PostgreSQL schemas follow authority and lifecycle

- Status: Proposed
- Date: 2026-11-21

## Context

Oz's [database review](../reviews/2026-11-21-oz-database.md) proposed a
logical PostgreSQL design for the data strategy (0043), and identified the
decisions that must precede any migration.

## Decision

Proposed, following the review:

**Schemas by authority and lifecycle**, not by feature.

| Schema | Holds | Lifecycle |
| --- | --- | --- |
| `record` | Signed operations, admission receipts, stream heads, segment and checkpoint references | Evidence; append-only, except stream heads |
| `state` | Waves, wavelets, documents, participants, permissions, bounds, search | Rebuildable projections |
| `identity` | Passkey public credentials, signing-key history, authentication metadata | Security-sensitive directory |
| `delivery` | Federation destinations, outbox, acknowledgements, retries | Operational bookkeeping |
| `ops` | Migrations, projection cursors, sealing jobs, deployment metadata | Local state |

**Operations and admissions are separate.** Signed envelopes are stored as
exact bytes (`bytea`); extracted columns are indexes into them, validated
against them. Admission receipts are the server's own records, never mixed
into what a participant signed.

**Position is a cursor.** A wavelet's admission position is maintained by the
application under a row lock, not by a database sequence. It means replay
order on that server, not CRDT order.

**Projections never own the record.** Each projection carries its source
cursor and format version. Nothing in `state` can cascade into `record`.

**Semantics stay in Rust.** CRDT and policy logic live in the shared Wave
implementation, behind the host storage interface; no triggers carry them.

**Privileges.** `wave_owner` owns; `wave_app` appends and reads the record,
and never updates, deletes, truncates or runs DDL. Migrations, sealing and
pruning run on separate privileged paths. Database permissions limit
accidents and a compromised runtime; they do not defend history against the
administrator. Witnessed checkpoints and sealed segments do.

**pgvector stays disabled** until a retrieval feature chooses a model.

**Sealing is crash-safe.** No tail entry is pruned until its sealed segment
is verified, and the authority for sealed segments must survive the loss of
PostgreSQL.

## Consequences

Before migrations are written, these must be decided (the review's list):
the operation's hash identity and envelope (0028, 0029); causal order versus
admission position (this record and 0005); the CRDT representation (0025);
federation authority (0005); erasure and commitments (0029, 0032); sealed
manifest authority; acknowledgement durability (0043); and migration role
separation.

The first implementation scope is operations, admissions, stream heads,
minimal projections and migration metadata, with the review's test list.
