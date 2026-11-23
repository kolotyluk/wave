# 0049. PostgreSQL handles durable delivery; external event brokers wait

- Status: Accepted
- Date: 2026-11-23

## Context

Wave needs reliable background processing and delivery to participants,
robots and federated servers. Kafka and other durable event streams could
distribute work to many independent consumers, but would introduce another
stateful system and a transaction boundary between it and PostgreSQL.

Eric has implemented exactly-once processing across Kafka and PostgreSQL
before, using PostgreSQL transactions to support the guarantee. That
experience informs this decision: a broker alone does not make database or
external effects happen exactly once.

## Decision

**Use PostgreSQL-backed durable delivery initially.** Kafka and other
external event brokers are not required infrastructure. Reconsider them
when measured throughput, backlog, independent replay, or consumer-isolation
needs justify the additional system.

The transactional-outbox boundary is the starting point: admitted evidence,
its receipt, correctness-critical state changes, and resulting delivery
obligations commit together. Workers process committed obligations, with
durable progress, retries, and failure reporting.

**Assume delivery can repeat.** Give events stable identities and make
consumers idempotent. Where consumer effects and progress share a PostgreSQL
database, commit them in the same transaction. External side effects need
their own idempotency or reconciliation mechanism; do not claim a general
end-to-end exactly-once guarantee.

Notifications may wake workers, but durable records and reconciliation
determine outstanding work. Ordering, backpressure, authorization at
delivery, and recovery must be explicit. A queue is not permission to
disclose protected content or to use stale authority state.

Keep delivery behind the host boundary, so an external broker can be added
without making the Wave protocol depend on it. Broker offsets would not
replace operation identities, admission cursors, or causal dependencies.

## Consequences

The initial deployment has fewer stateful dependencies. This is not a
decision to implement a general-purpose streaming platform inside Wave.

This decision concerns messaging and background delivery. It does not move
blobs or sealed history into PostgreSQL, replace federation with database
replication, or settle the remaining proposals in
[0043](0043-data-strategy.md) and [0045](0045-postgresql-schemas.md).

Detailed tables, worker leasing and transaction behavior remain to be
specified and tested. If a broker is added later, reliable publication from
the database and downstream duplicate handling still need explicit design.
