# Normal and Swarm participation profiles
Date: 2026-11-23
Status: Proposed high-level design for review, not an accepted architecture decision or an implemented capability.
## Purpose
Most Waves are expected to involve a modest working group. A public Wave could attract hundreds, thousands, or eventually millions of people. The design should accommodate that possibility without imposing large-system machinery on every deployment.

**Normal and Swarm are proposed participation profiles of one protocol, not separate protocols or trust levels.** A profile configures interaction, presence, subscription defaults, and resource budgets. It does not determine whether a Wave is public or private, who a person is, or what they may do.

This document outlines behavior and boundaries, not final wire formats, database tables, numerical capacity guarantees, or an implementation plan. In the terminology of [features](features.md), profiles are configurations of capabilities; any missing protocol capabilities need their own decisions.
## 1. Profiles at a glance
| Concern | Normal | Swarm |
| --- | --- | --- |
| Intended use | Small-group collaboration: a case, board, negotiation, or working group | Large-scale consultation, public discussion, or contribution gathering |
| Initial scale hypothesis | Around 32 active collaborators or fewer | Hundreds or more contributors; potentially a much larger audience |
| Presence | Individual presence where authorized and consented | Limited, scoped, or aggregated presence; no universal live roster |
| Interaction | Direct editing and discussion among a working group | Scoped editing plus separate submissions, threads, proposals, or moderated intake |
| Delivery default | Live updates within the participant's authorized working scope | Explicitly selected feeds, batches, digests, or pull-based catch-up |
| Client requirements | A detailed view of the small working group is practical | No requirement to enumerate all participants or receive the whole Wave |
| Resource management | Bounded even when the group is small | Bounded admission, processing, and fan-out with explicit overload behavior |
| Evidence and authorization | Shared protocol guarantees | The same guarantees, not a reduced standard |

**32 is a design hypothesis, not a protocol constant or a demonstrated limit.** Crossing it must not silently change permissions or trigger an automatic mode change. A participant count alone is not a capacity measure.

Normal does not mean unrestricted editing; Swarm does not mean anonymous or public. A large private institutional Wave can use Swarm delivery, while a small public Wave can use Normal interaction.
## 2. Separate the independent policies
Each Wave needs an explicit configuration covering several independent concerns:
- **Participation profile:** Normal or Swarm, with a version and declared settings.
- **Visibility:** public, restricted, and any authorized views and bounds.
- **Action permissions and assurance:** who may read, contribute, edit shared material, endorse, vote, moderate, or change policy, and what evidence of identity/authority each action requires.
- **Subscription:** which authorized updates a client follows, at what granularity and delivery cadence.
- **Operational budgets:** limits on connections, requests, computation, storage, outstanding work, and amplification.

These may be governed at different scopes. The proposal is a Wave-level profile with explicitly permitted wavelet-specific interaction and delivery settings. Scope inheritance and precedence require specification; local settings must not override enclosing confidentiality or authority restrictions.

Example: a public consultation could combine anonymous reading, identified or pseudonymous submissions, identity-checked voting, no individual presence display, and optional topic subscriptions. These choices are not implied by the Swarm label.
## 3. Define what is being counted
Distinguish:
- **Authorized actors:** people, institutions and robots with rights to perform particular actions; attribution records the signer and represented principal as appropriate.
- **Contributors:** actors or permitted anonymous sessions that submit material.
- **Observers:** readers, including anonymous readers of public material, without implying membership or write permission.
- **Subscribers:** clients requesting continuing updates; one actor can have several clients.
- **Active editors:** clients concurrently changing a particular shared object.

The final protocol must reconcile these working terms with its participant model. Observers and subscriptions are not additional root identities.

Measure contributor and observer volumes separately from concurrent edits, operation size, subscriptions, query cost, and fan-out. Reports must label whether a number counts accounts, presentations, sessions, robots, submissions, or verified distinct people. Do not present connection counts as public support.
## 4. Invariants shared by both profiles
Subject to the unresolved encoding, envelope, and storage proposals:
- A profile never bypasses signature, permission, assurance, or bound checks. Anonymous reading does not create unsigned write access.
- Operations retain their causal dependencies. Admission position is a server-local replay cursor, not CRDT conflict order.
- Preserve exact signed evidence, server admission receipts, and the distinction between them.
- Admission, correctness-critical state, and delivery obligations use the agreed transactional boundary. Authorization-sensitive decisions cannot rely on stale background projections.
- Acknowledgements state the durability reached; profile and audience size do not imply a durability level.
- Readers receive only authorized views. Presence, counts, summaries, indexes, and notifications are disclosures too.
- A profile cannot revoke or weaken existing agreement, notice, retention, or disclosure obligations by implication.
- Security and denial-of-service resistance apply at every scale, including attacks by validly authenticated actors.

These are continuity requirements, not claims that the currently skeletal implementation enforces all of them.
## 5. Normal interaction
Normal optimizes for a working group that can understand who else is participating and follow its shared activity.

Proposed behavior:
- Live updates for the current authorized working scope, with reconnect and incremental catch-up.
- Individual presence and editing indicators only where policy allows; offline status is not proof that someone has not received or read material.
- Direct collaborative editing where granted, using the shared CRDT semantics.
- Bounded paging and history retrieval even when the group is small.
- Independent treatment of receipt, application, display, and any formal acknowledgement of notice. A connected client is not proof that its principal read or legally received something.

The profile must remain usable when a participant is offline, uses an accessible or low-bandwidth door, or does not keep a live subscription.
## 6. Swarm interaction
Swarm optimizes participation without making each client track the entire audience or edit the same object.

Proposed behavior:
- Let contributors submit into authorized topics, threads, or individual contribution objects rather than defaulting everyone into one shared document.
- Reserve editing of a consolidated document for actors granted that capability. Being able to submit does not automatically grant shared-edit access.
- Treat a submission queue, a curated view, and the admitted record as different things. A moderator's publication choice must not silently erase an admitted contribution.
- Offer explicit topic/wavelet filters, paged history, bounded update batches, and snapshot-based catch-up.
- Use scoped presence or privacy-reviewed aggregates where useful, without requiring a globally distributed roster. Intrinsic robots remain declared and discoverable under the proposed governance model; aggregated human presence is not permission to hide system actors.
- Label generated summaries and curated views, including their source scope, checkpoint/cursor, and omissions. They are navigation aids, not substitutes for source evidence.

Millions of observers, thousands of occasional contributors, and thousands of simultaneous editors are separate workload scenarios. Supporting one does not establish support for the others.

A single busy wavelet may still bottleneck on admission or stream-head contention. Wavelets are potential concurrency units, not a claim that arbitrary sharding or distributed admission is already solved. Federation authority remains open under [0005](decisions/0005-authority-on-admission.md).
### Example: Automated Housing Advocate to systemic housing analysis
The Automated Housing Advocate could host a Swarm Wave where many people describe housing experiences, contribute documents, and discuss recurring problems. Authorized robots help organize that material and prepare source-linked reports for submission to the Federal Housing Advocate.

This is a proposed independent participation channel, not a claim of government adoption, affiliation, an agreed integration, or authority to speak for the Federal Housing Advocate. The office already receives public submissions and engages with people and communities; the relevant distinction is that it addresses systemic issues rather than providing individual assistance or remedies. See the Canadian Human Rights Commission's [submission guidance](https://www.chrc-ccdp.gc.ca/find-help/make-submission-federal-housing-advocate) and the office's account of [hearing directly from people](https://housing.chrcreport.ca/hearing-directly-from-people-across-canada.html).

An illustrative flow:
1. **Listen at individual scale.** People contribute through accessible doors, with assistance or declared delegation where appropriate. They need not follow the whole Swarm or subscribe after contributing. Private case material need not become public merely because the consultation is public.
2. **Obtain scoped permission.** Distinguish help with a person's own matter from permission to use selected material in systemic analysis, publish an extract, or send it to an outside office. A robot's access to a private case is not permission to include it in a public report. Any model capability processing it must respect the same bounds.
3. **Organize without deciding truth.** Robots propose themes, chronologies, links, apparent inconsistencies, and missing information. Preserve sources, uncertainty, corrections, conflicting accounts, and minority experiences. Similar submissions are not necessarily duplicate people; many submissions are not proof of prevalence, representativeness, or a legal violation.
4. **Prepare an inspectable report.** Bind the report to a source scope and checkpoint, identify the robots and processing versions, and distinguish submitted accounts, reviewed evidence, and analytical inference. Include coverage, exclusions, counting methods, and limitations. Authorized reviewers can trace claims to sources without publishing protected identities or attachments.
5. **Review and transmit under authority.** An authorized human reviews an externally submitted report; a robot may transmit it under an explicit, recorded delegation and applicable disclosure permissions. An institutional recommendation or finding is not manufactured by labelling a generated summary as official. External submission formats and acknowledgement arrangements require agreement or use of the office's existing channels.
6. **Keep the feedback path.** Record report versions, approval and transmission, recipient acknowledgement if obtained, later corrections, and any response received. Expose only authorized feedback to contributors; sending a report is not proof that the office accepted its conclusions or will provide an individual remedy.

Small cohorts, quotations, locations, and combinations of facts can re-identify people even when names are removed. Aggregation and publication need a privacy review; pseudonymity or a large audience alone does not make disclosure safe. Explain limits on withdrawal once material has been lawfully disclosed, rather than promising to recall it.

This example combines many bounded individual interactions with a smaller reviewed analytical output. It does not require millions of people to edit one document, and robots must not become the only means of surfacing rare or urgent experiences. Distinguish requests for immediate individual help from systemic reporting and offer appropriate human/service referral paths. Participation is self-selected, not automatically a representative survey; anonymous accounts cannot be counted as verified distinct people.
## 7. Identity, anonymity, and agreement
[0021](decisions/0021-identity-assurance-spectrum.md) attaches assurance to actions. [0047](decisions/0047-one-person-one-identity.md) distinguishes one root identity, its aliases and pseudonymous presentations, and anonymity as no identity claim.

Apply that distinction in both profiles:
- A pseudonym may be persistent, but its root linkage stays tightly protected; do not broadcast it in participant rosters, telemetry, public receipts, or event feeds.
- Anonymous reading claims no identity and does not require creating a fictional root identity.
- Anonymous contributions are an open design path, not enabled by this document. They still need a signature/envelope construction, admission permissions, resource controls, and a decision about agreement acceptance.
- An ephemeral signing key could establish continuity of a submission or session, but not a distinct human, lawful authority, or eligibility to vote. It must not be presented as identity verification.
- Anonymous actions cannot count as distinct people where identity-backed counting matters. Separately, a cryptographic signature can authenticate bytes without identifying a person.
- Robots remain attributable to their principals and cannot multiply the number of people represented by creating sessions or presentations.

ADR 0047 requires participation agreements, while some actions require no identity. Before anonymous contribution is implemented, resolve how agreement acceptance, attribution, abuse handling, and recovery work without silently changing either decision. Until resolved, anonymous viewing of authorized public material is distinct from anonymous writing.

Do not claim anonymity from the operator merely because no identity is requested. Network addresses, timestamps, and usage patterns can be identifying. Specify logging, retention, disclosure, and anti-linkage expectations.
## 8. Subscription and delivery
**A contribution must not require an ongoing subscription.** A non-subscribing contributor still needs an authorized way to retrieve admission status and any receipt. The receipt/status-access mechanism is open; it must not expose private submissions through guessable identifiers.

Subscriptions select authorized scope and delivery behavior; they neither grant permission nor change identity assurance. Revalidate disclosure rights for queued and cached delivery, and discard unauthorized delivery work without deleting underlying admitted evidence.

Start with PostgreSQL-backed durable delivery under [0049](decisions/0049-postgresql-backed-delivery.md):
- Commit admission evidence, required state, and resulting delivery obligations together.
- Use retryable workers, stable event identities, and idempotent consumers. Where effects and progress share a database, commit them together.
- Keep slow consumers off the admission critical path. Acknowledging admission does not wait for every subscriber to receive the event.
- Bound backlog, retries, feed sizes, and client buffers. Disconnect slow live consumers with explicit resumption behavior rather than buffering indefinitely.
- Consider shared authorized feeds and per-scope cursors instead of one durable outbox row per anonymous reader. Delivery bookkeeping is not evidence that an observer is a distinct person.
- Treat notifications as wake-up hints, not the durable queue.

Receipt and history replay remain available through their authorized retention/recovery paths when ephemeral delivery state expires. If a cursor predates retained delivery data, return a defined snapshot/history resynchronization path, not silently incomplete success. Sealing/pruning must preserve access to authoritative history.

Reconsider Kafka or another broker only when measured backlog, fan-out, throughput, or independent consumer needs justify it. A broker is optional host infrastructure, never an alternative permission system.
## 9. Snapshots, caching, and verification
Large audiences should not require every reader to fetch and replay the full history:
- Provide versioned, appropriately verifiable snapshots and incremental ranges under the eventual snapshot/proof specification.
- Bind materialized views and generated summaries to source progress and format versions.
- Check authorization before cache lookup/disclosure as appropriate; caches must distinguish policy/view versions and must not mix restricted and public data.
- Cache public material only when actually permitted to publish it. Private rosters, subscriptions, root mappings, and the existence of protected bounds must not leak through shared caches or aggregates.
- Do not describe integrity verification as proof that a supplied prefix is the latest complete history; freshness and rollback detection require separate checkpoint/witness rules.

Previously disclosed public material cannot be recalled merely by switching profile or invalidating a cache. Publication and later restriction need explicit policies consistent with the unresolved erasure design.
## 10. Profile changes
A profile change is proposed as an authorized, signed policy operation with a version, effective point, and recorded rationale. This requires envelope/policy details still to be specified.

Before applying one:
- Identify affected scopes, rights, subscriptions, pending contributions, and required notices or agreements.
- Preserve existing authority, consent, confidentiality, and history. Changing to Swarm does not make a Wave public; changing to Normal does not eject participants.
- Define how clients discover the new policy and recover from stale configuration. Authority-changing requests must be checked against current policy at admission.
- Continue to resolve admitted work and outstanding obligations; retries remain idempotent across the transition.

Crossing a scale target can prompt a recommendation or capacity response, not an automatic change in participation rights. Returning to Normal may require a smaller active editing group or different subscription defaults, but any restriction needs its own authorized policy and process. Historical contributors remain attributed.

Existing session limits and emergency backpressure may be enforced without a profile change. They must not be disguised as a new profile, a moderation finding, or proof of delivery.
## 11. Security and overload behavior
[0050](decisions/0050-attack-resistance.md) is an accepted cross-cutting requirement. Neither profile promises immunity from attack.

Separate controls for:
- Network/transport saturation, requiring suitable host/provider protection, including HTTP/3/UDP.
- Cheap ingress validation and bounded parsing, decompression, uploads and signature checks.
- Expensive valid requests: searches, exports, proofs, large histories and model calls.
- Distributed budgets across instances and per-deployment, scope, actor/session and workload limits.
- Amplification: one operation triggering many deliveries or recursively triggering robots.
- Storage, WAL, database connections, worker capacity, retries, logging, and long-lived clients.

Use budgets appropriate to actual cost; do not rely on signatures or IP addresses alone. Avoid a separate independent allowance for every alias or pseudonym, while protecting root linkage within the identity-service boundary. Exact allocation and fairness mechanisms remain open.

Show a truthful status: rejected before admission, pending if a defined pending mechanism exists, admitted at a stated durability, or uncertain after a connection failure. If admission may have committed, clients reconcile by stable identity rather than assuming rejection. Pending storage must itself be bounded and cannot be confused with the admitted record.

Under overload, protect correctness and confidentiality, bound queues, and reject or defer new work explicitly. Never silently lose admitted evidence or weaken durability. Rate limiting is an operational control; sanctions or eligibility changes require their own authority, notice, and appeal.
## 12. Evaluation and open decisions
No participant capacity is claimed until measured. Evaluate at least:
- A small working group with concurrent edits and offline reconnects.
- A large public audience with few writers.
- Bursts of independent contributions and a separate shared-document editing workload.
- Slow consumers, long outages, retries, replica failover, and profile changes.
- Large authorized histories and exports alongside admission traffic.
- Anonymous/Sybil traffic, malicious authenticated actors, and robot feedback loops.
- Disclosure correctness across permission changes, cached views, summaries and protected bounds.

Measure admission latency and throughput, connection/fan-out cost, backlog age, replay time, resource usage, failover behavior and fairness. Trust checks include signature/receipt integrity, no unauthorized disclosure, no duplicated effects, and no loss of acknowledged admitted evidence within the stated failure guarantee.

Decisions needed before implementation:
1. Profile scope, inheritance, configuration format, and change authority.
2. What the initial Normal target measures: active editors, actors, clients, or another workload measure.
3. Anonymous contribution envelopes, agreement acceptance, status recovery, and abuse controls.
4. Subscription filters, cursor scope, resumption, and any formal notice obligations.
5. Submission versus admission/publication/moderation states and their retention rules.
6. Snapshot/proof/freshness formats and protected-view caching.
7. A busy wavelet's admission/concurrency limits and the still-open federation authority model.
8. Resource budgets, documented overload policy, and evidence-based broker adoption criteria.

**Recommended direction for review:** one protocol; two configurable profiles; independent identity and subscription policies; PostgreSQL-backed delivery initially; the same trust and security requirements at every scale.
## References and decision status
- Accepted foundations: [admission](decisions/0005-authority-on-admission.md), [action assurance](decisions/0021-identity-assurance-spectrum.md), [one person, one identity](decisions/0047-one-person-one-identity.md), [PostgreSQL-backed delivery](decisions/0049-postgresql-backed-delivery.md), and [attack resistance](decisions/0050-attack-resistance.md).
- Relevant proposals: [envelope](decisions/0029-signed-operation-envelope.md), [verifiable views](decisions/0032-verifiable-views.md), [bounds](decisions/0036-bounds.md), [without prejudice](decisions/0037-without-prejudice.md), [agreement](decisions/0038-agreement.md), [data strategy](decisions/0043-data-strategy.md), and [PostgreSQL schemas](decisions/0045-postgresql-schemas.md).
- Intrinsic-robot details are under separate review in draft ADR 0048, not included here. Their eventual adoption is not implied by this participation-profile proposal.
- Overview: [architecture](architecture.md), [data](data.md), [features](features.md), and [robot governance](robot-governance.md).
