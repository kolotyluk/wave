# Google Wave and Canada Wave

How Canada Wave differs from the Wave Google built, and why. Canada Wave
stands on Google Wave's shoulders; this document says which ideas it keeps
and what it changes. The repository is the source of truth for the details,
and decisions are linked where they are recorded.

## What Google Wave was

Google announced Wave in May 2009: a shared, live document in which people
and robots worked together, with every change kept and the whole history
replayable. Google stopped developing it in 2010. It continued as Apache
Wave in the Apache incubator until it was retired in 2018.

Its model was sound and ahead of its time: waves made of wavelets, documents
changed only by operations, concurrent editing reconciled live, robots as
participants, federation between servers. Its interface showed people the
machinery and left them to work out what it was for, and too many could not.

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

| | Google Wave (2009) | Canada Wave |
| --- | --- | --- |
| **Purpose** | Real-time collaboration; "email reinvented" | Public collaboration infrastructure: work with a record that can stand as evidence |
| **Explained by** | Its technology | The journey: a housing letter, a consultation, a case ([messaging](messaging.md)) |
| **Front of house** | One rich client showing the machinery | A Front Door: an AI agent in plain language; a Side Door for the structure; phone-first ([0033](decisions/0033-default-doors-and-tabbed-app.md), [0034](decisions/0034-responsive-web-phone-first.md)) |
| **Convergence** | Operational transformation, ordered by the server | CRDTs; the server decides admission, not order ([0004](decisions/0004-crdt-convergence.md), [0005](decisions/0005-authority-on-admission.md)) |
| **The record** | History playback | Evidence: every operation signed, admission receipts chained, time-stamped and witnessed independently, exportable and verifiable without trusting the operator ([0022](decisions/0022-evidentiary-record.md), [0043](decisions/0043-data-strategy.md)) |
| **Encoding** | Protobuf, with its own JSON mapping | Deterministic CBOR signed with COSE; JSON only as a view ([0015](decisions/0015-cbor-and-cose.md), [0016](decisions/0016-media-type.md)) |
| **Transport** | Its own RPC over WebSocket; federation over XMPP | HTTP/3 and IPv6 as the ground floor; WebTransport for the stream ([0041](decisions/0041-http3-ground-floor.md), [0042](decisions/0042-ipv6-first.md)) |
| **Implementation** | Java on the server; Java compiled to JavaScript in the browser | Rust throughout, running as WebAssembly everywhere ([0002](decisions/0002-webassembly-everywhere.md), [0003](decisions/0003-rust-throughout.md)) |
| **Identity** | Google accounts | No one remembers a password; passkeys and keys; one person, one identity; private identifiers; an assurance spectrum from anonymous to verified ([0046](decisions/0046-passwords-scope.md), [0047](decisions/0047-one-person-one-identity.md), [0052](decisions/0052-authentication-keys.md), [0053](decisions/0053-person-identifiers-and-aliases.md)) |
| **Visibility** | Per wavelet: all or nothing | Bounds, verifiable views and without-prejudice protection, all of which still verify ([0032](decisions/0032-verifiable-views.md), [0036](decisions/0036-bounds.md), [0037](decisions/0037-without-prejudice.md)) |
| **Robots** | Extensions through an API | Accountable participants: sandboxed, acting for a named principal, governed by a published charter recorded with every act; AI agents among them ([0010](decisions/0010-robots-same-interface.md), [0011](decisions/0011-robot-sandbox.md), [0039](decisions/0039-robot-governance.md)) |
| **AI** | Not yet possible | The Front Door agent; MCP and A2A at the boundary; a Wave DOM for people and agents alike ([0035](decisions/0035-front-door-agent-is-a-robot.md), [0057](decisions/0057-mcp-and-a2a.md), [0058](decisions/0058-wave-dom.md)) |
| **Agreements and disputes** | Not modelled | Recorded agreements, robots agreeing for principals; disputes recorded and resolved, never overwritten ([0038](decisions/0038-agreement.md), [0054](decisions/0054-disputes.md)) |
| **Administration** | Google's own operations | The Admin Wave: the server administered through its first Wave ([0051](decisions/0051-admin-wave.md)) |
| **Storage** | Google's infrastructure | The log as the truth in object storage; PostgreSQL for derived state; any cloud, Canadian included ([0043](decisions/0043-data-strategy.md), [0049](decisions/0049-postgresql-backed-delivery.md)) |
| **Governance** | Owned by one company | An open protocol under any operator; published rules; deployments may mandate doors; a person can always see their own record ([0040](decisions/0040-deployments-may-mandate-doors.md)) |
| **Safety** | Not a stated goal | The most vulnerable require the most protection; when Wave fails, people can die ([0055](decisions/0055-mission-critical.md), [features](features.md)) |
| **Documentation** | Whitepapers and a draft federation spec | Four books and a Principles of Operation, with every MUST tested ([0044](decisions/0044-documentation.md)) |
| **Horizon** | Communication | Housing first; eventually a court proceeding conducted as a Wave ([0031](decisions/0031-courtroom-horizon.md)) |

## Why the changes

Three things are different now.

**AI can translate.** In 2009 nothing could stand between a person and the
structure and explain it. An AI agent now can, so the Front Door can hide the
complexity for convenience while the Side Door keeps it honest.

**The record has to be evidence.** Google Wave kept history for convenience.
Canada Wave keeps it so that a court, an auditor or a hostile expert can rely
on it. That one requirement drives the signatures, the encoding, the
witnesses, the export and much of the rest.

**It is public infrastructure.** A product can be withdrawn by its owner, as
Google Wave was. Infrastructure for the public must outlast any operator,
protect the people least able to protect themselves, and answer to law.
