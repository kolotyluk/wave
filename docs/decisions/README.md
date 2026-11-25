# Architecture decisions

Each decision is a numbered record. **Accepted** records are not rewritten; a
later record supersedes them. **Proposed** records are open for debate and are
not to be built on. See [0001](0001-record-architecture-decisions.md).

| No. | Decision | Status |
| --- | --- | --- |
| [0001](0001-record-architecture-decisions.md) | Record architecture decisions | Accepted |
| [0002](0002-webassembly-everywhere.md) | WebAssembly everywhere | Accepted |
| [0003](0003-rust-throughout.md) | Rust throughout | Accepted |
| [0004](0004-crdt-convergence.md) | Concurrent edits converge by CRDT | Accepted |
| [0005](0005-authority-on-admission.md) | The server is the authority on admission, not on order | Accepted |
| [0006](0006-server-as-webassembly-in-container.md) | The server runs as WebAssembly in a container | Accepted |
| [0007](0007-front-door.md) | The Front Door is its own pluggable component | Accepted |
| [0008](0008-side-door-early.md) | The Side Door is built early, in a minimal form | Accepted |
| [0009](0009-door-interface-modality-independent.md) | The door interface is independent of how the person communicates | Accepted |
| [0010](0010-robots-same-interface.md) | Robots join through the same interface as people | Accepted |
| [0011](0011-robot-sandbox.md) | Robots are isolated by the WebAssembly sandbox | Accepted |
| [0012](0012-browser-application.md) | The browser application hosts the doors | Superseded by 0033 |
| [0013](0013-apache-wave-lineage.md) | Apache Wave is lineage and reference, not runtime | Accepted |
| [0014](0014-invent-only-the-model.md) | Invent only the model; standardize everything else | Accepted |
| [0015](0015-cbor-and-cose.md) | Operations are encoded in deterministic CBOR and signed with COSE | Proposed |
| [0016](0016-media-type.md) | Wave's media type is application/wave+cbor | Accepted |
| [0017](0017-protocol-definition.md) | Rust is the working protocol definition; test vectors are the neutral truth | Accepted |
| [0018](0018-asking-and-changing.md) | Asking is separate from changing; asking uses HTTP QUERY | Accepted |
| [0019](0019-component-interface.md) | The robot and door boundary is described in WIT | Proposed |
| [0020](0020-no-passwords.md) | No passwords, ever | Accepted |
| [0021](0021-identity-assurance-spectrum.md) | Identity assurance is a spectrum | Accepted |
| [0022](0022-evidentiary-record.md) | The record is evidentiary | Accepted |
| [0023](0023-http-api-cbor.md) | The API is HTTP with CBOR bodies, not gRPC | Accepted |
| [0024](0024-live-stream-transport.md) | The live stream starts on WebSocket, with WebTransport to be tested | Superseded by 0041 |
| [0025](0025-crdt-library.md) | Which CRDT library | Proposed |
| [0026](0026-server-webassembly-runtime.md) | Which WebAssembly runtime hosts the server | Proposed |
| [0027](0027-reference-browser.md) | Firefox is the reference browser | Proposed |
| [0028](0028-identifiers.md) | How Waves, participants, operations and versions are identified | Proposed |
| [0029](0029-signed-operation-envelope.md) | The signed-operation envelope | Proposed |
| [0030](0030-one-image-through-stages.md) | One container image moves through every stage | Accepted |
| [0031](0031-courtroom-horizon.md) | The architecture contemplates replacing the courtroom | Accepted |
| [0032](0032-verifiable-views.md) | Views are verifiable projections of the record | Proposed |
| [0033](0033-default-doors-and-tabbed-app.md) | Default doors, replaceable shells, and a tabbed app | Accepted |
| [0034](0034-responsive-web-phone-first.md) | One responsive web app, phone-first for the Front Door | Proposed |
| [0035](0035-front-door-agent-is-a-robot.md) | The Front Door agent is an ordinary robot | Proposed |
| [0036](0036-bounds.md) | Bounds: visible demarcations with declared rules | Proposed |
| [0037](0037-without-prejudice.md) | Without prejudice is a kind of bound | Proposed |
| [0038](0038-agreement.md) | Agreement is a capability of participants | Proposed |
| [0039](0039-robot-governance.md) | Robots are governed by enforced constraints and a published charter | Proposed |
| [0040](0040-deployments-may-mandate-doors.md) | A deployment may mandate its doors | Proposed |
| [0041](0041-http3-ground-floor.md) | HTTP/3 is the ground floor | Proposed |
| [0042](0042-ipv6-first.md) | IPv6 first | Proposed |
| [0043](0043-data-strategy.md) | The log is the truth; everything else is referenced or derived | Proposed |
| [0044](0044-documentation.md) | Documentation to the standard of the Principles of Operation | Proposed |
| [0045](0045-postgresql-schemas.md) | PostgreSQL schemas follow authority and lifecycle | Proposed |
| [0046](0046-passwords-scope.md) | No one has to remember a password; infrastructure avoids them too | Accepted |
| [0047](0047-one-person-one-identity.md) | One person, one identity | Accepted |
| [0048](0048-server-identity-and-intrinsic-robots.md) | Servers have identities; intrinsic robots are not shown by default, never hidden | Proposed |
| [0049](0049-postgresql-backed-delivery.md) | PostgreSQL handles durable delivery; external event brokers wait | Accepted |
| [0050](0050-attack-resistance.md) | Attack resistance applies to every Wave | Accepted |
| [0051](0051-admin-wave.md) | The Admin Wave: a server is administered through a Wave | Proposed |
