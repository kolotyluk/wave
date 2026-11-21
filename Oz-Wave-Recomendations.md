# Oz: Revised Wave Database Recommendations
Date: November 21, 2026
Status: Revised proposal for discussion with Eric and Claude; not an accepted architecture decision.

## Scope and decision status
This revision incorporates [Claude's feedback](docs/reviews/Claude-Oz-Feedback.md). The [original dated review](docs/reviews/2026-11-21-oz-database.md) remains unchanged as a historical record.

The repository now distinguishes:
- **Accepted:** [ADR 0046](docs/decisions/0046-passwords-scope.md), clarifying participant and infrastructure authentication; and [ADR 0047](docs/decisions/0047-one-person-one-identity.md), establishing root identities, protected pseudonyms, declared delegation, and accountable participation.
- **Proposed:** [ADR 0043](docs/decisions/0043-data-strategy.md), the storage strategy; [ADR 0045](docs/decisions/0045-postgresql-schemas.md), adopting the logical schema recommendation; and the identifier and envelope decisions in [ADR 0028](docs/decisions/0028-identifiers.md) and [ADR 0029](docs/decisions/0029-signed-operation-envelope.md).
- **Still to decide:** exact encoding, hash identity, signature composition, CRDT representation, federation authority, erasure construction, manifest authority, acknowledgement policy, and migration implementation.

The previous review's wording concerns about order, erasure guarantees, durability levels, outdated PostgreSQL deployment information, and password scope have been addressed in the project documents. They are not outstanding documentation contradictions. Implementation details below remain recommendations, not implicit amendments to ADRs.

This is a design review, not executable DDL or an implementation plan. It changes no database, ADR status, deployment, or repository history. The deployment tasks in Claude's feedback require separate authorization.

## Current state
- The workspace contains only the dependency-free `crates/wave-protocol`, implementing `PROTOCOL_VERSION = 0` and `ParticipantKind::{Person, Robot}`. There are no signed envelopes, persistence adapters, migrations, or conformance vectors yet.
- As recorded in [environments](docs/environments.md), TrueNAS runs PostgreSQL **18.7**, with pgvector **0.8.7** available but not enabled.
- Database `wave` exists; `wave_owner` is a non-login ownership role; `wave_app` is a restricted login without ownership-role membership or schema/temporary-table creation privileges.
- No application schemas or tables exist. Generated development credentials are outside the repository, consistent with ADR 0046.
- There is no synchronous replica. The present installation can claim only a commit on one server.

## 1. Retain the five-schema boundary
Keep the principle in [data](docs/data.md): **the log is the truth; blobs are referenced by hash; everything else is derived and rebuildable.**

Divide PostgreSQL by authority and lifecycle:
- **`record`:** immutable participant evidence, immutable admission receipts, segment/checkpoint references, and narrowly mutable stream heads.
- **`state`:** rebuildable Wave, wavelet, document, membership, permission, bound, agreement, and retrieval projections.
- **`identity`:** protected root directory, aliases, public credentials and signing-key history, pseudonym bindings, and identity administration projections.
- **`delivery`:** federation destinations, transactional outbox, acknowledgements, and retry state.
- **`ops`:** migration history, projection progress, sealing jobs, and local recovery bookkeeping.

These are not five equally public schemas. In particular, ordinary Wave reads must not expose the identity directory. Local authentication challenges, bootstrap trust, and delivery attempts also need their own recovery and retention policies; they are not necessarily reconstructible from participant operations.

Keep CRDT and authorization semantics in shared Rust behind the WIT host-storage interface, not SQL triggers. PostgreSQL stores and constrains records; it does not define Wave's protocol. Keep database dependencies out of `wave-protocol`.

## 2. Operation identity: hash the canonical signed payload
I agree with Claude's direction: **prefer an operation ID derived from the canonical signed payload, rather than the entire COSE envelope**, subject to ADR 0029 and conformance vectors.

This separates the identity of an asserted action from the particular collection of signatures presented as evidence for it. A later principal attestation can refer to the same action without renaming it.

The protocol must specify:
- Exact canonical CBOR bytes, including rejection rules for alternative encodings.
- A versioned, domain-separated hash construction identifying this as a Wave operation, with an algorithm-qualified identifier.
- Signed protocol context: target scope, actor/presentation, action, causal dependencies, and any other fields that affect interpretation or authority.
- Which COSE protected headers and external authenticated context are required. Unprotected headers must never determine authorization.
- Whether any interpretation-relevant protected-header information must also be bound into the payload identity. Two envelopes with one payload ID must not acquire contradictory meanings.
- Required signatures at admission, permitted algorithms, key resolution, and the semantics of additional attestations.

### Preserve all evidence, not just one mutable envelope
Proposed logical tables:
- **`record.operations`:** operation hash, exact canonical payload bytes, protocol version, and validated extracted indexes.
- **`record.operation_envelopes`:** envelope hash, operation hash, exact COSE bytes, and format/version. Multiple immutable envelopes may refer to one operation.
- **`record.attestations`:** optional separately signed statements referring to an operation or envelope, when the protocol chooses that representation.
- **`record.admissions`:** the admitting authority's receipt, referencing the operation and the exact envelope/evidence set accepted at admission.

These names and columns are illustrative, not settled migrations. Store evidence as `bytea`; derived `jsonb` is not a substitute. Never reconstruct signed bytes from extracted columns or overwrite an envelope to add a signature.

COSE signature composition needs an explicit choice: adding signatures to `COSE_Sign`, converting `COSE_Sign1`, and issuing a separate countersignature/attestation are not interchangeable. Existing signatures must retain their original to-be-signed context.

**A later signature does not retroactively authorize an earlier admission.** Evaluate principal authority, delegation, charter, required signatures, and revocation under the applicable admission policy. A later ratification is a new recorded act with its own effect, not a rewritten receipt.

## 3. Admission, replay cursors, and projections
Operations retain their causal dependencies. Admission position is only a particular server's replay cursor, as already clarified in ADRs 0005, 0043, and 0045.

An admission should identify the admitting server/deployment, target wavelet, local position, operation and accepted evidence, observed time, policy/build versions, and previous receipt hash. A server's observed time is not an independent timestamp.

Include the admitting stream/server identity in position uniqueness unless the stream identifier itself unambiguously includes it. Define federation idempotency before choosing a unique constraint: a local retry must not duplicate an admission, while another server's receipt is distinct evidence.

Use `record.stream_heads` for the current cursor and receipt hash, updated under a row lock or compare-and-swap. It is mutable append-control state, not immutable evidence or CRDT order. Use an application-managed counter, never a database sequence as the source of protocol identity.

An admission transaction should:
1. Validate encoding, signatures, dependencies, and candidate authority in Rust.
2. Lock or otherwise serialize the relevant stream and authorization state.
3. Revalidate the authority-state version; retry if it changed.
4. Append operation evidence and receipt, update required projections and stream head, and enqueue required delivery atomically.
5. Commit, then acknowledge the durability actually reached.

Cross-wavelet authority requires coordinated locks/version checks and defined retry/lock ordering; one wavelet lock is not automatically sufficient. Rejected admission must not leave an admitted record or delivery work behind. Any retained rejection evidence needs an explicit separate policy.

Projections carry source cursors/checkpoints and format versions. Use versioned CRDT state and preserve attributed replay data, not just rendered text. Permission-sensitive reads cannot use stale authorization merely because content projections are eventually consistent.

Use application-generated UUIDv7 for evolving entities if ADR 0028 is adopted; signers choose identifiers and PostgreSQL stores them. Do not freeze today's two participant variants into a permanent SQL enum before institution/principal representation is settled. Nothing in `state` may cascade-delete `record` evidence.

## 4. Identity: one root, protected presentations, recorded authority
ADR 0047 is accepted. The schema must implement its distinction between a person, their credentials, their presentations, and their representatives.

### Root directory and aliases
Recommended logical families in `identity`:
- **`root_identities`:** root identifier and entity kind, with lifecycle/status references.
- **`aliases`:** email, phone, and other identifier claims, verification provenance, validity periods, and root binding.
- **`public_credentials`:** WebAuthn credential identifiers, relying-party context, public keys, and enrollment/revocation history.
- **`signing_keys`:** historical public signing keys and authority/rotation/revocation references.
- **`assurance_evidence`:** scoped evidence references and verification provenance, with expiry and access controls.

Multiple passkeys and addresses bind to one root; changing them does not create another person. Shared/recycled addresses and phone numbers require explicit rules: an alias match is evidence to investigate, not proof that two people are identical. Apply uniqueness constraints only where the alias type and verification policy justify them.

Avoid unnecessary raw identity evidence. Protect personal identifiers at rest, restrict matching to the identity service, and use keyed lookup tokens where appropriate rather than unsalted hashes of enumerable phone/email values. Keys remain outside PostgreSQL; metadata, access patterns, and lookup tokens still need protection.

Assurance belongs to action requirements and supporting evidence, not a timeless scalar that turns a person into universally “verified.” A verified credential's duplicate resistance depends on issuer coverage, enrollment, recovery, and cross-issuer policy; it does not by itself prove global uniqueness.

### Pseudonyms and anonymity
Use **`identity.pseudonym_bindings`** for protected links between presentations and roots, with scope, validity, and disclosure authority. Public Wave projections refer to the authorized presentation, not a convenient root foreign key exposed through joins.

Root linkage must not leak through general history exports, logs, metrics, search, embeddings, receipts, or reusable public signing keys. Consider presentation-scoped keys and claims; their issuance and audit must not publish the hidden link. Root-link disclosure follows the person's choice or lawful process, as ADR 0047 requires, and is itself access-controlled and audited.

Anonymous participation makes no identity claim. Do not assign anonymous actions invented root identities that can count as distinct people for voting, endorsement, or signatures. Define how agreement requirements apply to anonymous access before implementing that path.

### Merges and correction
Use **`identity.merge_events`** and a rebuildable resolution projection backed by recorded operations. Preserve original identifiers, aliases, provenance, and historical signer attribution; never bulk-rewrite old operations to the surviving root.

The mistaken-merge reversal protocol remains open in ADR 0047. Recommend an explicit correction/split operation rather than destructive edits, with rules for reassigning credentials, agreement bindings, delegations, and duplicate-sensitive counts. Do not enable automatic irreversible merges before those rules exist. Matching and appeal evidence must not become public linkage evidence.

### Delegates, robots, institutions, and agreements
- Institutions have one identity with many authorized actors; robots bind to principals. Neither is a second human identity.
- Record delegations with delegator, delegate, scope, authority evidence, required assurance, validity, revocation, and any permitted subdelegation.
- Every represented action distinguishes the actual signer from the party represented. Possessing an alias or passkey is not a substitute for delegation.
- Store signed agreement acceptance referencing an immutable version/hash of the terms, accepting actor, represented party where applicable, authority evidence, and admission receipt.
- Record enforcement evidence, lawful-authority references, notices, responses, decisions, restrictions, appeals, and corrections. Do not implement automatic fines or legal penalties from a duplicate-detection score.

`state.delegations`, `state.agreements`, and enforcement views are current projections. Their signed source events belong in appropriately protected record streams. General logs must not contain plaintext identity evidence or pseudonym links: an append-only privacy leak cannot be repaired with a filtered UI.

The identity directory needs a separate access boundary, even if initially hosted in this database. Do not grant every `wave_app` connection unrestricted directory reads merely because a schema is named `identity`. Choose a narrowly scoped identity-service role/interface, protected evidence storage, and explicit recovery procedure. Directory associations can be sensitive even when the cryptographic keys themselves are public.

## 5. Sealed manifest authority outside PostgreSQL
**Recommendation:** immutable, signed, versioned, content-addressed manifests stored beside segments, independently copied and anchored by witnesses. PostgreSQL catalogs them; it is not their sole authority.

Each manifest should bind:
- Protocol/format version, issuing authority and signing-key identifier.
- Stream identity, covered admission range, segment hash/length, and boundary receipt hashes.
- Previous manifest/checkpoint hash and the required continuity/dependency information.
- Snapshot and artifact inventory references needed for verification/recovery, without disclosing protected material.
- Timestamp and independent witness statements over the exact specified manifest/checkpoint digest.

Define a non-circular signing/attestation format: timestamp/witness proofs may live in separately hashed bundles referring to the manifest, with a final checkpoint binding those bundles. Specify precisely what each signature proves.

A mutable `latest` object is a discovery hint, never evidence of completeness. Content hashes prove integrity, not that the newest prefix has been supplied. Recovery needs independently retained checkpoint heads/witness observations and trusted issuer/key-history material outside the lost database. Signature verification alone cannot detect presentation of a valid old prefix.

Witnesses and timestamp authorities have different roles: a timestamp can establish that a digest existed by a time; rollback/equivocation detection requires the agreed witness protocol, retained heads, and comparison. A witness does not necessarily store the underlying bytes.

### Crash-safe sealing and restoration
Seal a stable prefix, upload content-addressed segments, verify availability/hash/retention, publish and verify manifests and required proofs, then finalize local metadata. Only a separate authorized maintenance path may prune redundant tail bytes afterward.

Make every transition resumable and idempotent. PostgreSQL and S3 do not share a transaction. Incomplete uploads are not authoritative checkpoints; orphan handling must never remove referenced evidence.

After database loss, recover trusted heads and key history, verify manifests and continuity, fetch and verify segments/artifacts, reconstruct retained indexes and stream heads, and replay versioned projections. Treat snapshots as validated acceleration, not independent authority. Preserve enough operation metadata after pruning for references, idempotency, and history queries.

This recovers the **sealed prefix** only. Unsealed entries require surviving PostgreSQL, replica, or suitable backups/WAL; do not promise their recovery from manifests that never covered them.

## 6. PostgreSQL backups and PITR
**Recommendation:** evaluate **pgBackRest first**, with WAL-G as an alternative, against PostgreSQL 18, the TrueNAS deployment model, and the selected S3-compatible backend. Do not adopt both without a concrete need.

The baseline is recoverable physical base backups plus continuous WAL archiving to encrypted storage in an independent failure domain. Start and verify archiving before the first base backup. Logical dumps are useful supplemental exports, not a PITR chain.

Specify RPO/RTO and retention before choosing schedules. Test archive continuity, failed-archive alerts, restore points/timelines, capacity, encryption-key recovery, and supported tool/server versions. Retain every WAL segment needed by retained base backups. Object storage on the same TrueNAS pool alone is not disaster recovery.

Keep ZFS snapshots as supplemental fast rollback. They are not a substitute for independently stored backups or demonstrated PostgreSQL-consistent recovery. Back up configuration, role/grant definitions, extension/version requirements, and separately protected recovery/bootstrap material as well.

PostgreSQL backups contain sensitive directory information and may retain plaintext removed from current projections. Apply access controls, retention, legal-hold and erasure policy to backups too. Backup encryption keys and signing private keys belong in host/KMS/HSM management, never database rows.

### Recover PostgreSQL and object storage coherently
Before committing an operation that requires an artifact, establish the artifact's required durability and verify its hash. Track orphan uploads rather than allowing committed references to nonexistent blobs.

Retain object versions and manifests needed by all supported database recovery points. A restored database must not advertise unavailable artifacts or unverified sealed ranges. Recovery should validate references, reconcile restored cursors with authoritative manifests, rebuild projections, and resume outbox delivery idempotently.

Run isolated restore drills that demonstrate a chosen point-in-time target, sealed-history verification, missing-object detection, and safe delivery resumption. Backups provide operational recovery; they are not independent evidence against an administrator who can rewrite both database and backup.

## 7. Synchronous replication: beta capability, not a label
`wave1` and `wave2` are federation peers, not replicas of one another. Separate databases and roles on one PostgreSQL instance provide logical isolation, not hardware resilience.

**Recommendation:** establish a synchronous standby on an independent machine, storage, and power/failure domain before any beta admission promises level 2 durability or becomes the only copy of consequential user data. Same-host containers or a TrueNAS VM cannot provide that guarantee.

A second physical host with independent storage/power is a useful first step; a remote site/cloud also protects against site loss, with latency, privacy, jurisdiction, and cost trade-offs. Choose location against the actual failure guarantee, not simply “another VM.”

It is reasonable to run disposable federation tests first with explicit level 1 acknowledgements. Do not silently upgrade the durability claim because the deployment is called beta. No particular replica host or provider is selected by this review.

Configure PostgreSQL synchronous standby selection and per-transaction policy deliberately:
- `synchronous_commit = on`, with a qualifying synchronous standby configured, waits for durable remote WAL flush; without one configured, it does not create remote durability.
- `remote_write` is not equivalent to durable standby flush.
- `remote_apply` additionally waits for replay/visibility, and is needed only when that stronger read guarantee is required.

On standby loss, level 2 acknowledgements must wait or fail according to policy. Any downgrade must be explicit and reported, not hidden as a successful level 2 commit. A client timeout while waiting can leave an uncertain commit outcome; retry by operation identity rather than assuming rollback.

Test failover, fencing/split-brain prevention, replay lag, replication-slot disk pressure, promotion and rejoining. Synchronous replication is not a backup or protection from replicated logical corruption.

Level 3 additionally requires the specified sealing, timestamping, and independent witnessing. If that claim includes recovery after host loss, segment/manifest storage must survive the claimed failure; a witnessed hash alone cannot restore lost bytes.

## 8. Migration ownership and runtime privileges
**Recommendation:** a separate migration login/process, authorized to explicitly `SET ROLE wave_owner`, never the runtime login.

Keep `wave_owner` non-login. For the migration identity, configure membership so ownership privileges are not inherited automatically but explicit role switching is permitted; in PostgreSQL 18 verify membership options such as `INHERIT FALSE` and `SET TRUE`. `NOINHERIT` alone is not a complete privilege audit.

Use short-lived migration credentials or certificates, operator/CI approval, and isolated execution. A migration role capable of becoming owner is privileged even when it does not normally inherit that authority. Do not grant that membership, its credentials, or equivalent role-switch paths to `wave_app`.

Migrations should acquire a migration lock, verify checksums/version history, act under the intended owner, and apply transactional DDL where supported. Mark nontransactional operations explicitly; plan maintenance and expand/contract compatibility rather than assuming every DDL change can run online or roll back.

Set owner-specific default privileges under the role that creates objects, and grant existing objects explicitly. Revoke inappropriate `PUBLIC` access, including default function execution at the appropriate global default-privilege scope; per-schema revokes cannot remove globally granted defaults.

Runtime grants should allow only:
- Required evidence reads/appends; no historical update/delete, `TRUNCATE`, ownership, or DDL.
- Narrow stream-head updates and required projection/delivery writes.
- Identity access through its chosen restricted boundary, not blanket schema-wide reads.

Sealing/pruning has another restricted maintenance path; it is not a reason to grant deletion to the application. Avoid `SECURITY DEFINER` helpers unless necessary; review ownership, fixed safe `search_path`, parameterization, and execution grants when used.

SQL permissions constrain accidents and a compromised runtime, not a superuser or host administrator. RLS can add defense in depth but does not replace Rust authorization; pooled connections require safe transaction-local context and reset behavior.

## 9. Production authentication
ADR 0046 resolves the old scope question: participants never authenticate with passwords/PINs, generated infrastructure passwords are acceptable in development, and production must replace them with an appropriate alternative.

**PostgreSQL recommendation:** verified TLS with client-certificate authentication on tightly restricted `hostssl` rules and certificate-to-role mappings. The client must also verify the server identity, for example with libpq `sslmode=verify-full`; server-side client-certificate validation does not perform that client-side check.

Separate application, identity, migration, replication, and backup identities as needed. Define issuance, expiry, revocation, renewal overlap, compromise response, pool reconnection, and monitoring. Store private keys outside PostgreSQL and source control with narrowly scoped filesystem/KMS access.

Database client-certificate trust and issuance need an explicit design. Public HTTPS server certificates in the environment documentation do not automatically provide suitable database client identities. Do not assume the browser certificate policy settles the internal database trust model.

**Cloud workload identity:** evaluate only after choosing a provider and database offering. Determine whether workload identity directly authenticates to PostgreSQL, obtains a short-lived database token, or accesses a secret/certificate issuer. It is not a universal passwordless PostgreSQL feature; some products still pass ephemeral tokens through a password-shaped API.

Document the chosen mechanism against ADR 0046, including renewal, least-privilege mapping, bootstrap, and outage recovery. Do not publish port 5432 to the internet as a shortcut; use private connectivity and verified encrypted connections.

## 10. Erasure, bounds, and retrieval
The corrected erasure wording is appropriate: destroying a blob key can make retained ciphertext unreadable, but cannot recall disclosed plaintext or remove copies in snapshots, backups, indexes, embeddings, or exports.

[ADR 0029](docs/decisions/0029-signed-operation-envelope.md) and [ADR 0032](docs/decisions/0032-verifiable-views.md) still need to define commitments, encryption, salts, disclosure proofs, and key lifecycles. The same question applies to alias links, merge evidence, delegations, and agreement records; putting sensitive plaintext into an immutable identity event is not solved by encrypting its directory projection.

Do not reduce bounds to booleans. Preserve overlapping scope, historical ranges, exclusions, consent, and flow restrictions in versioned projections.

Full-text search, embeddings, and generated summaries remain rebuildable `state` data, with source hash/version, checkpoint, model/version/dimensions, chunking, and visibility context. Enforce authorization before disclosing content, scores, snippets, or derived information. External model calls are outflows too. Keep pseudonym-root mappings out of general retrieval.

Leave pgvector disabled until a real feature chooses a model and index strategy.

## 11. First implementation boundary and review agenda
Retain ADR 0045's narrow first scope: operation evidence, admissions, stream heads, minimal Wave/wavelet/participant projections, and migration metadata. If payload-based identity is adopted, preserve envelope variants from the beginning. Implement accepted identity/agreement requirements before enabling participation paths that rely on them.

Add durable delivery before federation tests, and sealing/recovery/witnessing before stronger evidentiary claims. Defer detailed document/bound tables and embeddings until protocol shapes exist.

The original eight pre-migration topics remain, with more precise status:
1. **Envelope and identity:** payload hashing is recommended here; exact bytes and signature/attestation semantics await ADR 0029.
2. **Causal order versus position:** the conceptual distinction is documented; stream identity, constraints, and federation replay rules still need specification.
3. **CRDT representation:** settle attributed replay and conformance vectors.
4. **Federation authority:** settle admission authority, duplicate receipts, refused dependencies, and recovery.
5. **Erasure and commitments:** settle the cryptographic construction and exact guarantees.
6. **Manifest authority:** consider the independently retained, signed, witnessed manifest design above.
7. **Acknowledgement durability:** levels are documented; choose stage-specific policy, remote storage guarantees, and outage behavior.
8. **Ownership and migrations:** consider explicit migration-role switching and the restricted maintenance paths above.

ADR 0047 adds implementation questions, not a reopening of its policy: duplicate matching, correction of mistaken merges, scoped presentation credentials, protected disclosure, and anonymous-access/agreement interaction.

Validation should cover byte-for-byte vectors; envelope variants without rewritten evidence; non-retroactive authority; duplicate/retried and uncertain-outcome commits; concurrent authorization changes; atomic rollback/outbox behavior; deterministic replay; crashes at each sealing transition; pruning refusal; database-free sealed-prefix recovery and rollback detection; PITR/object consistency; replica outage/failover; privacy-preserving identity access and merge correction; and runtime-role inability to rewrite history or perform DDL.

## References
- [Original review](docs/reviews/2026-11-21-oz-database.md) and [Claude's response](docs/reviews/Claude-Oz-Feedback.md).
- [Data strategy](docs/data.md), [environments](docs/environments.md), and [architecture](docs/architecture.md).
- [ADR 0043](docs/decisions/0043-data-strategy.md), [ADR 0045](docs/decisions/0045-postgresql-schemas.md), [ADR 0046](docs/decisions/0046-passwords-scope.md), and [ADR 0047](docs/decisions/0047-one-person-one-identity.md).
- [Identifiers](docs/decisions/0028-identifiers.md), [signed envelopes](docs/decisions/0029-signed-operation-envelope.md), and [verifiable views](docs/decisions/0032-verifiable-views.md).
- PostgreSQL 18: [PITR and WAL archiving](https://www.postgresql.org/docs/18/continuous-archiving.html), [standby replication](https://www.postgresql.org/docs/18/warm-standby.html), [certificate authentication](https://www.postgresql.org/docs/18/auth-cert.html), and [default privileges](https://www.postgresql.org/docs/18/sql-alterdefaultprivileges.html).
