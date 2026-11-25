# 0028. How Waves, participants, operations and versions are identified

- Status: Proposed
- Date: 2026-11-19

## Context

Every path in the API, every signature and every reference between servers
depends on how things are named. Apache Wave had a specification for wave
identifiers.

## Decision

Open. To be settled before the endpoints are more than drafts.

Proposed so far:

- **Immutable things are named by the hash of their content**: operations,
  blobs, sealed segments, snapshots. A hash proves what something is.
- **Evolving things are named by UUIDv7** (RFC 9562): Waves, wavelets,
  participants, robots, bounds, deployments. UUIDv7 begins with a timestamp,
  so new keys arrive in order and database indexes stay compact. GUID keys
  also make import and export straightforward.
- **People and robots are the exception: random UUIDs (version 4)**, kept
  private, with pairwise identifiers in each context
  ([0053](0053-person-identifiers-and-aliases.md)).
- **No auto-increment keys**, anywhere.
- **The database never generates identifiers.** A new Wave, wavelet, bound or
  participant comes into being through a signed operation, so its identifier
  is inside that operation, chosen by the signer. Identifiers are generated in
  Wave's own code (the Rust `uuid` crate), in the browser and on the server
  alike; PostgreSQL stores them in its native `uuid` type.

Still open: the human-readable and network form of identifiers (how a Wave on
`wave1.ztuph.com` is named to `wave2`), and how identifiers survive
federation.

## Consequences

Precedes 0023 and the first test vector.
