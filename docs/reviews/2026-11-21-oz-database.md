# Oz: Wave Database Recommendations
Date: November 21, 2026
Status: Proposal for discussion with Eric and Claude—not an accepted architecture decision.
## Purpose and scope
This review proposes a logical PostgreSQL persistence design grounded in the existing repository. It does not define final DDL, change the protocol, or authorize implementation. Eric decides and verifies; Claude designs and develops; Oz assists with deployment and operations, as described in `README.md`.
No database changes or application schema migrations were performed as part of this review.
## Current implementation and deployment
- The workspace currently contains only `crates/wave-protocol`.
- Its implemented protocol model consists of `PROTOCOL_VERSION` and `ParticipantKind::{Person, Robot}`.
- There are no operation envelopes, persistence adapters, migrations, or protocol test vectors yet.
- TrueNAS now runs PostgreSQL **18.7**, with pgvector **0.8.7** available.
- The `wave` database exists.
- `wave_owner` is a non-login ownership role.
- `wave_app` is a restricted login role. It can connect to `wave` and use the existing `public` schema, but cannot create schemas or temporary tables, administer roles, or create databases. It is not a member of `wave_owner`.
- No application schemas, tables, or extensions have been created.
- Application credentials are kept outside this repository. Do not add passwords, connection strings containing passwords, or private keys to source control.
## Main recommendation
Keep the `wave` database and divide tables by **authority and lifecycle**, rather than by interface feature.
The proposed rule in `docs/data.md` is the right foundation:
> The log is the truth. Blobs are referenced by hash. Everything else is derived and rebuildable.
Do not begin with conventional editable document tables and add auditing afterward. Persist signed records first; treat current documents, permissions, and search results as projections.
The storage strategy is still **Proposed** in ADR 0043. The design below is conditional on its acceptance and on resolution of the identifier, envelope, CRDT, and federation questions.
## Proposed PostgreSQL schemas
| Schema | Responsibility | Lifecycle |
| --- | --- | --- |
| `record` | Signed operations, admission receipts, checkpoint and segment references | Evidence-bearing; historical records are append-only |
| `state` | Current Waves, wavelets, documents, participants, permissions, bounds, and search | Rebuildable projections |
| `identity` | Passkey public credentials, signing-key history, and authentication metadata | Security-sensitive directory |
| `delivery` | Federation destinations, outbox, acknowledgements, and retries | Mutable operational bookkeeping |
| `ops` | Migration history, projection cursors, sealing jobs, and deployment metadata | Local implementation state |
These boundaries distinguish primary evidence from derived data and operational bookkeeping. They are not substitutes for Wave's participant-level authorization.
Not everything operational is reconstructible from the participant-operation log. Authentication challenges, delivery attempts, bootstrap trust, and similar local state need explicit lifecycle and recovery policies.
## 1. Separate signed operations from admission records
What a participant signs and what an admitting server records are different objects. Keeping them separate prevents server-generated metadata from being mistaken for participant-signed content.
### `record.operations`
Conceptual fields:
- `operation_hash`: primary key, with an algorithm-qualified representation.
- `protocol_version`.
- `envelope_bytes`: exact encoded signed envelope.
- Extracted fields needed for indexing, once the envelope is defined.
Store signed CBOR/COSE as `bytea`, not reconstructed JSON. A `jsonb` interpretation can be derived, but must never replace the original bytes.
The identifier decision must define exactly which bytes are hashed: payload, signed envelope, or another explicitly specified object. Do not assume these are interchangeable.
Avoid prematurely fixing a digest length or signature algorithm. ADRs 0028 and 0029 should establish those.
Extracted columns must be validated against the signed bytes. They are indexes into the record, not a second authoritative account of its contents.
### `record.admissions`
Conceptual fields:
- `wavelet_id`.
- `log_position`.
- `operation_hash`.
- Admitting server/deployment identifier.
- Server-observed admission time.
- Rules/version reference used for admission.
- Server build reference.
- Receipt hash and prior receipt reference, if adopted by the protocol.
Suggested constraints:
- Unique `(wavelet_id, log_position)`.
- An idempotency constraint preventing duplicate admission of the same operation to the same wavelet.
- References to retained operation metadata.
The exact idempotency key depends on how signed operations bind their target wavelet and how federation handles admissions. These semantics must be defined before enforcing them in DDL.
Server-observed time is not an independently trusted timestamp. Keep that distinction explicit.
### Resolve the meaning of order
`log_position` should be an admission/replay cursor, **not a CRDT conflict-resolution order or global clock**.
There is a terminology tension in the current documents:
- Accepted ADR 0005 says the server is the authority on admission, not on order.
- The proposed storage strategy assigns operations a place in a wavelet's log order.
Define separately:
1. CRDT causal dependencies and convergence semantics.
2. A particular server's local admission sequence.
3. Any canonical ordering or receipts required by federation and evidentiary export.
Do not insert a server-chosen prior-log hash into an already participant-signed envelope. If the server adds a chain link, it belongs in a separately defined admission record or receipt.
### Stream heads
A small `record.stream_heads` table can hold each wavelet's current position and receipt hash for transactional appends.
This is mutable append-control metadata. Unlike operations and receipts, it is not itself an immutable historical record.
Use an application-managed counter under a row lock or compare-and-swap—not `SERIAL`, identity columns, or database-generated entity identifiers. A position is a cursor, not an operation's identity.
This follows the intent of ADR 0028, but the interpretation should be agreed explicitly.
## 2. Rebuildable application state
Possible future table families in `state`:
| Table family | Purpose |
| --- | --- |
| `waves`, `wavelets` | Current structure and relationships |
| `participants`, `memberships` | People/robots and participation |
| `documents`, `blips` | Materialized CRDT state and readable content |
| `grants`, `delegations` | Current permission and acting-for projections |
| `bounds`, `bound_scopes`, `agreements` | Governed regions, history ranges, and consent |
| `robot_charters` | References to the charter versions robots act under |
Each projection should carry its source checkpoint/cursor and projection-format version, so the system can identify stale state and rebuild it.
Use application-generated UUIDs for evolving entities if ADR 0028 is adopted. PostgreSQL can store them in its native `uuid` type without generating them.
Do not reduce bounds to `is_private` or `without_prejudice` booleans. The proposed model includes overlapping scopes, exclusions, historical ranges, inherited rules, and different agreement requirements.
Do not use today's two-value `ParticipantKind` as a permanent database enum without resolving institutions and principals. ADR 0029 already identifies that gap.
Avoid making the immutable record depend on the continued existence of mutable projection rows. For example, removing a participant from a current membership projection must not cascade-delete their operations.
## 3. Admission transaction and concurrency
For the first PostgreSQL-backed server:
1. Validate encoding, signatures, dependencies, permission, and bounds in the Wave implementation.
2. Begin a database transaction and coordinate access to the relevant stream head.
3. Recheck that the authority-state version used for validation is still current.
4. Append the operation and admission receipt.
5. Update the required projections and enqueue federation delivery.
6. Commit; acknowledge afterward.
Permission changes and ordinary edits must not race past one another. Signature validation alone is insufficient if authorization was checked against stale state.
If authorization spans several wavelets or bounds, a single wavelet lock may not be sufficient. Define the concurrency mechanism for the relevant authority state, including lock ordering or retry behavior.
Retried submissions must not duplicate operations, projection updates, or outbox work.
Keep CRDT and policy semantics in the shared Rust implementation—not PostgreSQL triggers—consistent with the portable host-storage interface in `docs/data.md`.
An append function may enforce database-level atomicity and invariants, but that is not a replacement for cryptographic or policy validation.
Any deferred projection processing needs an explicit consistency boundary. Permission-sensitive reads must not use stale projections merely because eventual rebuilding is possible.
## 4. Sealed history, snapshots, and artifacts
Later additions to `record` can include segment, checkpoint, witness, timestamp, snapshot, and artifact-reference tables.
Store:
- Content hashes and format versions.
- Object-storage references.
- Covered wavelet log ranges.
- Verification and sealing status.
- References to independently retained timestamp and witness proofs.
Do not store private signing keys or plaintext artifact-encryption keys in PostgreSQL.
### Crash-safe sealing
Sealing needs a recoverable, idempotent workflow:
1. Select and identify a stable log prefix.
2. Build and upload the segment.
3. Verify its content hash and required durability/retention properties.
4. Obtain the attestations required by the chosen protocol.
5. Finalize the segment manifest and checkpoint.
6. Only then consider removing redundant tail bytes under a separate maintenance policy.
Never discard the only durable copy of a tail entry before its sealed replacement is verified. Ordinary SQL transactions cannot atomically commit to PostgreSQL and S3 together.
Retain enough operation and range metadata to satisfy indexes, references, and history queries after tail bytes are pruned. Final manifest authority and restoration rules should not depend solely on a PostgreSQL row if the database is supposed to be reconstructible after loss.
### Erasure is a protocol question
Append-only plaintext signed content and lawful erasure are incompatible without an agreed cryptographic design.
A later `redacted` flag does not remove plaintext from old log segments, snapshots, backups, search indexes, embeddings, or exports.
ADRs 0029 and 0032 need to establish what is signed, what is committed, what is encrypted, who can access salts and keys, and how destruction and verification coexist.
Destroying an encryption key does not erase copies of plaintext already disclosed to recipients. Technical and legal guarantees must be stated accurately.
## 5. Search and pgvector
Keep full-text indexes and embeddings in `state`, as rebuildable data.
Embedding records should identify:
- Source content hash and version.
- Model/version and embedding dimensions.
- Chunking/version metadata.
- Wavelet and applicable visibility context.
- Source checkpoint or projection cursor.
Filter authorization before disclosing results or snippets. Similarity scores, embeddings, and cached summaries can leak restricted information too.
Information derived from a without-prejudice bound must retain applicable flow restrictions. Sending content to an external embedding model is also an outflow requiring authorization.
Do not assume all embedding models produce the same vector dimensions. Select a storage/index strategy only when the first retrieval feature chooses a model.
Leave pgvector disabled until there is a concrete retrieval feature.
## 6. Identity and credentials
Participant authentication follows the accepted passwordless direction:
- Store public WebAuthn credentials and their participant bindings.
- Keep public signing-key history sufficient to verify historical operations after rotation or revocation.
- Keep private keys outside PostgreSQL, as required by ADR 0043.
- Define the authority and recovery policy for enrollment, key rotation, revocation, and bootstrap trust.
Public keys are not secret cryptographic material, but directory associations and credential metadata may still be sensitive. They are not automatically public to every participant.
Do not collapse identity assurance into a permanent scalar on a person. ADR 0021 attaches assurance requirements to actions; the selected framework and evidence model remain open.
## 7. Database permissions
Keep:
- `wave_owner` as the non-login object owner.
- `wave_app` as the runtime login, with no ownership, role-management, or DDL privileges.
As tables appear, grant:
- Required append/read access to record tables, not update/delete.
- Necessary projection and delivery permissions.
- No `TRUNCATE` or schema-creation privileges.
The stream-head update path is an explicit exception to historical immutability and should have narrowly scoped permissions.
Sealing/pruning and migrations should use separate privileged paths. Migration execution must deliberately act as the intended owner so future default privileges apply consistently.
Do not give the application membership in its ownership role merely to simplify migrations.
PostgreSQL permissions reduce accidents and constrain a compromised runtime role. They do not make history tamper-proof against the database administrator; independently witnessed checkpoints and externally retained sealed records address that stronger threat.
Row-level security may provide defense in depth, but it cannot replace Wave's cryptographic projections or information-flow enforcement. A pooled service role requires carefully scoped, transaction-local authorization context if RLS is used.
## 8. Runtime, durability, and environment isolation
The WebAssembly component should depend on the small host storage interface, not directly on PostgreSQL SQL or a specific cloud SDK.
PostgreSQL persistence belongs behind the host adapter. Do not add storage dependencies to `wave-protocol`.
For federation testing, use separate databases and credentials for `wave1` and `wave2`, not merely schemas in one application database, as proposed in `docs/environments.md`.
Sharing one PostgreSQL server still shares infrastructure, availability, and administrator trust. It does not fully test independent failure domains.
The existing single TrueNAS PostgreSQL instance does not yet satisfy ADR 0043's proposed synchronous-replica requirement for an unsealed primary tail.
Define acknowledgement durability explicitly:
- A committed transaction on one server.
- A transaction durably acknowledged by a synchronous replica.
- A record additionally sealed and independently witnessed.
These are different guarantees. Do not describe the first as the third.
## 9. Suggested first implementation scope
Once the blocking protocol choices are settled, start with:
- Operations.
- Admissions.
- Stream heads.
- Minimal Wave/wavelet/participant projections.
- Migration metadata.
Add durable delivery before federation testing. Add sealing, recovery, timestamping, and witnessing before claiming independently verifiable durable history.
Defer detailed CRDT document tables, bounds, and embeddings until their protocol shapes exist.
Tests should cover:
- Duplicate submissions and retry idempotency.
- Concurrent appends and authorization changes.
- Rejection without record or outbox side effects.
- Transaction rollback without partially visible admission.
- Replay rebuilding the same projections.
- Crashes at every sealing transition.
- Refusal to prune an unverified or unavailable sealed prefix.
- Authorization-filtered history and search.
- Application-role inability to rewrite evidence or perform DDL.
- Byte-for-byte export verification against protocol test vectors.
## 10. Decisions to review before writing migrations
1. **Exact signed envelope and hash identity:** which bytes define an operation?
2. **Causal order versus admission-log position:** what does the server sequence mean?
3. **CRDT representation:** what must be retained for complete attributed replay?
4. **Federation authority:** whose admissions are final, and what happens to refused dependencies?
5. **Erasure and commitments:** which content is permitted in immutable storage?
6. **Sealed manifest authority:** what survives independently of PostgreSQL?
7. **Durability level:** what must happen before an acknowledgement is sent?
8. **Role and schema ownership:** how are migrations separated from runtime?
These decisions are expensive to retrofit. Schema names are not.
## Documentation follow-ups
- Update `docs/environments.md`: it still describes PostgreSQL 17.12. The separate instance was recreated as PostgreSQL 18.7 with pgvector available.
- Clarify whether ADR 0020, “No passwords, ever,” concerns participant authentication or also infrastructure credentials. The current database adapter login uses a generated password. If infrastructure passwords are also prohibited, choose an alternative such as appropriately configured certificate authentication.
- Reconcile “the server does not decide order” with the storage documents' per-wavelet ordering language.
- Preserve the distinction between accepted decisions and proposals. This document does not resolve the open envelope, identifier, CRDT, federation, or erasure decisions by implication.
## Source documents reviewed
- `README.md`
- `Cargo.toml`
- `crates/wave-protocol/Cargo.toml`
- `crates/wave-protocol/src/lib.rs`
- `vectors/README.md`
- `docs/architecture.md`
- `docs/data.md`
- `docs/environments.md`
- `docs/features.md`
- `docs/robot-governance.md`
- ADRs 0004, 0005, 0015, 0019, 0020, 0021, 0022, 0025, 0028, 0029, 0030, 0032, 0036, 0037, 0038, 0039, and 0043.
