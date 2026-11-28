# Google Wave and Canada Wave

How Canada Wave differs from the Wave Google built, and why. Canada Wave
stands on Google Wave's shoulders; this document says which ideas it keeps
and what it changes. The repository is the source of truth for the details,
and decisions are linked where they are recorded.

**Status:** Canada Wave is a design and walking skeleton, not a shipped
replacement for Google Wave. The comparisons below describe accepted design
requirements and proposals, not implemented guarantees. ADRs 0056–0058
remain Proposed. Historical and external-protocol facts were checked on
2026-11-28 against the primary sources listed below.

## What Google Wave was

Google announced a developer preview of Wave on **May 28, 2009**: a shared,
live conversation/document with replayable history and robot extensions.
On **August 4, 2010**, Google announced that it would stop developing Wave
as a standalone product; this was not the service's shutdown date. Google
subsequently announced read-only operation from **January 31, 2012**, and
shutdown on **April 30, 2012**.

The open-source continuation overlapped the hosted service: Apache Wave
entered incubation on **December 1, 2010**, and retired on **January 15,
2018**, without graduating from the incubator. Google's hosted product,
the federation protocol, and Apache's implementation are related but not
interchangeable historical subjects.

Its model was sound and ahead of its time: waves made of wavelets, documents
changed only by operations, concurrent editing reconciled live, robots as
participants, federation between servers. Our assessment is that an
interface exposing much of that machinery made the purpose difficult to
discover. That is a design interpretation, not a proven single cause of
failure: Google's announcement cited insufficient user adoption.

## What Canada Wave keeps

- **The shared object.** One evolving record that several participants change
  together.
- **Operations as the only way to change it,** attributed and replayable.
- **Waves and wavelets,** with wavelets as compartments.
- **Robots as participants,** not plug-ins working behind the scenes.
- **Federation:** Waves shared between independent servers.
- **Stand-off annotations** over documents.

([0013](decisions/0013-apache-wave-lineage.md))

## What Canada Wave changes

The historical column includes the original protocol and Apache lineage
where indicated. It must not imply that later Apache implementation choices
all shipped in Google's May 2009 preview.

| | Google Wave and its lineage | Canada Wave design |
| --- | --- | --- |
| **Purpose** | Real-time communication and collaboration | Public collaboration infrastructure: work with a record designed to support evidentiary scrutiny |
| **Explained by** | Product, platform and protocol demonstrations | Journey-first framing: a housing letter, a consultation, a case ([messaging](messaging.md)) |
| **Front of house** | Google's rich web client, plus APIs for embedding and extensions | A Front Door: an AI agent in plain language; a Side Door for the structure; phone-first ([0033](decisions/0033-default-doors-and-tabbed-app.md), [0034](decisions/0034-responsive-web-phone-first.md)) |
| **Convergence** | OT; the wavelet host transforms and sequences operations for that wavelet | CRDTs; the server decides admission, not CRDT conflict order ([0004](decisions/0004-crdt-convergence.md), [0005](decisions/0005-authority-on-admission.md)) |
| **The record** | History playback; provider/domain signatures and history hashes in federation | Participant-signed evidence plus separate chained admission receipts; independent timestamping/witnessing and offline verification are design requirements ([0022](decisions/0022-evidentiary-record.md), [0043](decisions/0043-data-strategy.md)) |
| **Encoding** | Structured XML documents and annotations; Protocol Buffers federation payloads | Proposed deterministic CBOR signed with COSE; JSON as a view or envelope carrier, not a replacement for signed bytes ([0015](decisions/0015-cbor-and-cose.md), [0016](decisions/0016-media-type.md)) |
| **Transport** | Product-specific client/server communication; the published federation protocol extends XMPP | Proposed HTTP/3 and IPv6 ground floor; WebTransport stream, with host compatibility layers ([0041](decisions/0041-http3-ground-floor.md), [0042](decisions/0042-ipv6-first.md)) |
| **Implementation** | GWT browser client; Java-based open-source server lineage | Rust throughout, compiled to WebAssembly components ([0002](decisions/0002-webassembly-everywhere.md), [0003](decisions/0003-rust-throughout.md)) |
| **Identity** | Google accounts for the hosted product; provider-domain addresses and provider authentication in federation | Passwordless participant authentication; one root identity, private identifiers and action-specific assurance, with key/delegation details still proposed ([0046](decisions/0046-passwords-scope.md), [0047](decisions/0047-one-person-one-identity.md), [0052](decisions/0052-authentication-keys.md), [0053](decisions/0053-person-identifiers-and-aliases.md)) |
| **Visibility** | Per wavelet: all or nothing | Bounds, verifiable views and without-prejudice protection, all of which still verify ([0032](decisions/0032-verifiable-views.md), [0036](decisions/0036-bounds.md), [0037](decisions/0037-without-prejudice.md)) |
| **Robots** | Named participants extended through a robot API | Named participants with explicit principal attribution, sandboxed capabilities and a proposed versioned charter recorded with each act ([0010](decisions/0010-robots-same-interface.md), [0011](decisions/0011-robot-sandbox.md), [0039](decisions/0039-robot-governance.md)) |
| **AI** | NLP-based contextual spelling and robot demos such as translation; not today's LLM interfaces | Proposed conversational Front Door, MCP/A2A adapters, and semantic views with bounded, permission-scoped agent metadata ([0035](decisions/0035-front-door-agent-is-a-robot.md), [0057](decisions/0057-mcp-and-a2a.md), [0058](decisions/0058-wave-dom.md)) |
| **Agreements and disputes** | Collaboration model, rather than the proposed legal agreement/dispute lifecycle | Proposed recorded agreements and disputed claims resolved under authority without overwriting history; legal effect is not created by a label ([0038](decisions/0038-agreement.md), [0054](decisions/0054-disputes.md)) |
| **Administration** | Google's service operations; separately operated open-source servers | Proposed Admin Wave, retaining a host-level recovery boundary ([0051](decisions/0051-admin-wave.md)) |
| **Storage** | Google-hosted service; open-source providers could use other storage | Proposed PostgreSQL primary unsealed tail plus projections/outbox; sealed record and blobs in object storage, behind the host adapter ([0043](decisions/0043-data-strategy.md), [0049](decisions/0049-postgresql-backed-delivery.md)) |
| **Governance** | Google controlled its hosted product; the protocol was open and independent servers were possible | Intended public infrastructure under independently accountable operators; published rules and inspectable records, not a claim of official government adoption ([0040](decisions/0040-deployments-may-mandate-doors.md)) |
| **Safety** | General-purpose collaboration | Explicit mission-critical production requirement; the skeleton does not yet meet it ([0055](decisions/0055-mission-critical.md), [features](features.md)) |
| **Documentation** | Engineering whitepapers and draft federation specifications | Planned four books and Principles of Operation, with tested normative requirements ([0044](decisions/0044-documentation.md)) |
| **Horizon** | Communication | Housing first; eventually a court proceeding conducted as a Wave ([0031](decisions/0031-courtroom-horizon.md)) |

## Why the changes

Three things are different now.

**Conversational assistance has changed.** Wave already demonstrated
natural-language processing and translation in 2009; AI itself is not new.
Today's LLM-based assistants make a conversational Front Door more
plausible, while the Side Door preserves direct inspection. Whether that
helps people use Wave safely remains a design and testing question.

**The intended evidence model differs.** The original federation protocol
already used cryptography; it should not be described as an unsigned log.
Canada Wave's design adds participant-level attribution, distinct admission
receipts, independent checkpoint witnessing and verifiable disclosure.
These aim to support scrutiny by a court or hostile expert; neither a
signature nor the design itself guarantees truth, completeness or legal
admissibility.

**It is public infrastructure.** A product can be withdrawn by its owner, as
Google's hosted Wave service was. The open-source continuation shows why
that is different from withdrawing an entire open protocol. Canada Wave
aims for portability, continuity beyond an operator, protection for
vulnerable people, and accountability to law.

## MCP and A2A: what the boundary actually means

MCP connects AI applications to resources and tools; A2A connects independent
agents through messages and tasks. They are complementary, not interchangeable.
MCP was introduced on **November 25, 2024**. A2A v1.0 shipped on **March 12,
2026**; the Linux Foundation's **April 9, 2026** announcement describes
its first-anniversary adoption milestones, not the v1.0 release date.

MCP uses JSON-RPC, with standard bindings including stdio and Streamable
HTTP. A2A v1.0 has a Protocol Buffers data model and JSON-RPC, gRPC and
HTTP+JSON bindings. Agent Cards support discovery; a verified card does not
itself confer Wave permissions, delegation or agreement.

Both are now hosted by AAIF under the Linux Foundation: MCP was a founding
project in **December 2025**; AAIF announced A2A's hosting in **August 2026**.
AGENTS.md, also hosted there, is an instruction convention, not another wire
protocol or an enforcement mechanism.

Wave adapters still need compatible versions, authorization, exact signed
envelopes, lifecycle/retry mappings, and disclosure controls. External JSON
signatures, such as signed Agent Cards, keep their native verification
rules; they do not replace Wave's proposed CBOR/COSE signatures. See
[0057](decisions/0057-mcp-and-a2a.md) for the proposed boundary, not a claim of
plug-and-play interoperability.

## Sources

- Google: [developer-preview announcement, May 28, 2009](https://googleblog.blogspot.com/2009/05/went-walkabout-brought-back-google-wave.html).
- Google: [standalone development decision, August 4, 2010](https://googleblog.blogspot.com/2010/08/update-on-google-wave.html).
- Google: [2011 announcement of the January 31 / April 30, 2012 service milestones](https://googleblog.blogspot.com/2011/11/more-spring-cleaning-out-of-season.html).
- Apache: [incubation status, including entry and retirement dates](https://incubator.apache.org/projects/wave.html).
- Google protocol authors, archived by Apache: [July 2009 federation specification](https://svn.apache.org/repos/asf/incubator/wave/whitepapers/federation/wavespec.html), including document annotations, OT, provider identity and cryptographic signatures. This is historical reference, not adopted runtime code.
- Google: [I/O engineering sessions, June 4, 2009](https://developers.googleblog.com/en/google-wave-google-io/), describing GWT, Spelly's NLP and the Rosy robot demo.
- MCP/A2A: [primary sources and specification links in ADR 0057](decisions/0057-mcp-and-a2a.md#sources-checked).
- Linux Foundation: [April 9, 2026 A2A anniversary/adoption announcement](https://www.linuxfoundation.org/press/a2a-protocol-surpasses-150-organizations-lands-in-major-cloud-platforms-and-sees-enterprise-production-use-in-first-year).
