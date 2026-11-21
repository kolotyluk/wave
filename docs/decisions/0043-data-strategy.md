# 0043. The log is the truth; everything else is referenced or derived

- Status: Proposed
- Date: 2026-11-20

## Context

Wave must run on any cloud and on local hardware (0030), keep a record that
can be evidence (0022), allow lawful erasure (0029, 0032), and scale to a
national deployment.

## Decision

Proposed: **the log is the truth, blobs are referenced by hash, and everything
else is derived and rebuildable.** Details are in [data](../data.md).

- **The log.** Two records, kept apart (see the [database
  review](../reviews/2026-11-21-oz-database.md) and 0045):
  - **Operations**: exactly what participants signed, linked by their causal
    dependencies. The server never adds anything to them.
  - **Admission receipts**: what the admitting server records about each
    operation it admits: its position in that server's admission sequence,
    the time the server observed, the rules and build it admitted under, and
    a hash link to the previous receipt. The position is a replay cursor,
    not the CRDT's order; the server decides admission, not order (0005).
  The unsealed tail of both is appended in PostgreSQL in one transaction. It
  is periodically sealed into immutable segment files in object storage,
  where it is checkpointed, time-stamped and witnessed (0022), and locked
  against change where the storage supports it.
- **Blips and documents** are derived state: the current result of the
  operations, materialized in PostgreSQL for fast reading and rebuildable from
  the log. Snapshots are sealed into object storage at checkpoints, each
  verifiable by hash against the log.
- **Artifacts** (attachments, audio, video, exports, evidence packages) are
  blobs in object storage, named by the hash of their content and referenced
  from operations by that hash. Each blob is encrypted with its own key.
  Destroying the key makes the stored blob unreadable while the record still
  proves something was there. It does not recall copies already disclosed to
  recipients, nor plaintext that reached snapshots, backups, search indexes
  or embeddings; how erasure and verification coexist is to be settled in
  0029 and 0032, and stated exactly.
- **Storage is the host's business.** The Wave component uses a small storage
  interface of its own, defined in WIT (append to the log, read a range, put
  and get a blob), which the host implements. WASI's storage proposals are to
  be adopted as they mature.
- **Backends.** Object storage through the S3 API; PostgreSQL for the tail and
  derived state. Locally, an S3-compatible server (Garage or SeaweedFS, to be
  evaluated) beside PostgreSQL.
- **Scaling is by federation.** No national database: many servers, each the
  home of its own Waves, each with its own PostgreSQL and a streaming replica
  for failover.
- **Private keys never go in a database.** They belong in a key-management
  service or hardware security module.

**Durability is stated, never implied.** Three levels are distinct:

1. committed on one server;
2. committed and durably acknowledged by a synchronous replica;
3. sealed, time-stamped and independently witnessed.

What a server acknowledges must say which level was reached. A development
server on a single PostgreSQL instance reaches only the first.

## Consequences

Losing a database loses no part of the sealed record: derived state is
rebuilt by replay.

A Wave exports as its sealed segments, snapshots and blobs, all named by hash
or UUID.

The skeleton can start with plain files behind the storage interface, then
move to S3 and PostgreSQL without changing the component.
