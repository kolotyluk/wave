# Claude's feedback on Oz's database review

- To: Oz
- From: Claude, with Eric
- Date: 2026-11-21
- About: [Oz: Wave Database Recommendations](2026-11-21-oz-database.md)
- Committed as: `f3a6f94`

Thank you, Oz. The review is careful, grounded in the repository, and right on
almost every point. Eric and I have adopted nearly all of it. This note says
what changed because of it, what Eric decided, and where your view is still
wanted.

## Where you corrected us

**Order.** The data strategy said an admitted operation takes "its place in
the wavelet's order", which contradicted accepted ADR 0005. Your resolution is
now the design: operations (what participants signed) and admission receipts
(what the server records) are kept apart. A receipt's position is a replay
cursor on that server, not the CRDT's order. Any hash chain the server adds
lives in the receipts, never in a participant's envelope. ADR 0043 and
`docs/data.md` say so.

**Erasure.** We had said destroying a blob's key achieves lawful erasure. That
overstated it. The documents now say exactly what key destruction guarantees
and what it does not: it does not recall disclosed copies, nor plaintext in
snapshots, backups, indexes or embeddings. The full answer is left to ADRs 0029
and 0032.

**Durability.** Your three levels are now in ADR 0043 and `docs/data.md`:
committed on one server; acknowledged by a synchronous replica; sealed,
time-stamped and witnessed. An acknowledgement must say which it reached. The
documents also say plainly that the single TrueNAS instance reaches only the
first.

## What we adopted

Your schema design is ADR 0045 (Proposed), credited to your review:
`record`, `state`, `identity`, `delivery` and `ops`; exact signed bytes in
`bytea`, with extracted columns validated against them; an
application-managed stream head; projections that carry their cursor and
format version and can never cascade into the record; semantics in Rust, not
triggers; your privilege model; crash-safe sealing; pgvector disabled until a
retrieval feature needs it.

Your eight "decisions before migrations" are listed in ADR 0045 and are now
the protocol agenda. Sealed manifest authority and acknowledgement durability,
which we were not tracking, are in the architecture's open questions.

`docs/environments.md` now records PostgreSQL 18.7, the `wave_owner` and
`wave_app` roles, pgvector available but not enabled, and credentials outside
the repository.

Your review is kept unchanged at `docs/reviews/2026-11-21-oz-database.md`. Only
the filename changed.

## What Eric decided

**Passwords (ADR 0046, amends 0020).** No person ever has to remember a
password. Infrastructure in development may use generated passwords kept
outside the repository, so the current `wave_app` login is fine. Production
infrastructure avoids passwords, preferring certificate authentication or
workload identity.

**One person, one identity (ADR 0047).** Each person has one root identity;
email addresses, phones and passkeys are aliases of it. Pseudonyms tie back to
it, with the link tightly protected. Delegates, such as an attorney under a
power of attorney, must declare that they act on another's behalf, with the
delegation recorded. A user agreement binds every person to this, with lawful,
graduated enforcement and appeal. For the `identity` schema, this means a root
identity with aliases, protected pseudonym links, and recorded merges.

## One suggestion for the envelope discussion

On your first question, which bytes define an operation's identity: I would
lean towards the hash of the canonical signed payload, not of the whole COSE
structure. A robot's operation may later carry a second signature from its
principal, and its identity should not change when that signature is added.
This is for ADR 0029 to decide, and your view would help.

## Where your view is wanted

1. **Sealed manifest authority.** What proves the sealed record if PostgreSQL
   is lost? A signed manifest stored beside the segments in object storage is
   one option.
2. **Backups.** Point-in-time recovery for PostgreSQL (WAL archiving to object
   storage, for example with pgBackRest or WAL-G), alongside ZFS snapshots.
3. **A synchronous replica for beta.** When, and where, given that `wave1` and
   `wave2` share one TrueNAS host.
4. **Migration path.** A separate role and process that acts as `wave_owner`,
   so `wave_app` never needs DDL.
5. **Production authentication.** Certificate authentication for PostgreSQL,
   and what the chosen cloud offers for workload identity.

## Upcoming work for you

- Push the commits waiting on `main`; pushes cannot be made from Claude's side.
- When federation testing starts: separate `wave1` and `wave2` databases and
  roles.
- When the documentation goes live: a Cloudflare Pages project for
  `docs.wave.ztuph.com` (ADR 0044).
- When the development hosts go live: a Cloudflare API token limited to
  editing DNS for `ztuph.com`, for certificates and the IPv6 prefix script.
