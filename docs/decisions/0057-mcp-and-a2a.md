# 0057. MCP and A2A meet Wave at its boundary

- Status: Proposed
- Date: 2026-11-27

## Context

Two complementary open protocols address different integration needs. MCP
connects AI applications to tools and data; A2A connects independent agent
systems, with Agent Cards for discovery and a task/message model.

Anthropic introduced MCP on November 25, 2024. A2A's first stable release,
v1.0, shipped on March 12, 2026. MCP joined the Agentic AI Foundation (AAIF)
at its December 9, 2025 formation; AAIF announced A2A's
hosting on August 17, 2026. Both are now AAIF projects under the Linux
Foundation, alongside AGENTS.md, but retain distinct specifications.

MCP uses JSON-RPC with bindings such as stdio and Streamable HTTP. A2A v1.0
defines a Protocol Buffers data model and JSON-RPC, gRPC, and HTTP+JSON
bindings. It is not a JSON-only transport.

The discussion paper (Appendix B) already lists A2A and MCP interactions
among the things an agent's actions must account for.

## Decision

Proposed:

**MCP can expose a door for agents.** A host adapter may expose authorized
Wave resources and tools: read a wavelet, query history, propose an operation.
MCP is an application protocol, not merely transport and not Wave's admission
or authorization model. Read-only requests remain read-only (0018).

**A protocol request is not a principal's signature.** Bind the authenticated
caller, represented principal, Wave signing key, scope and delegation
explicitly. Accept an exact signed Wave envelope, or have an authorized
participant construct and sign one under an actual delegation. An adapter
must not silently use the server's identity to impersonate the caller.
Ordinary signature, current-permission, assurance and bound checks still
decide admission (0005, 0010). A proposed operation is not an admitted one.

**A2A must not become an unrecorded negotiation bypass.** Wave-governed robots
representing different parties record substantive inter-party content as
ordinary signed operations, not hidden metadata. Outgoing intent precedes
external delivery; results, failures and uncertain outcomes are recorded
appropriately afterward. Use the outbox/retry boundary (0049); an A2A task
status, request ID, cancellation or transport acknowledgement is not a Wave
admission receipt or a rollback of an admitted act.

Not every discovery request, read or progress event changes a Wave.
When external content is imported, preserve its actual provenance: a local
robot can attest that it received a message, but cannot manufacture the
remote author's signature. Wave cannot prevent independent external parties
from talking elsewhere or retrospectively prove what they omitted.

A remote opaque A2A service is an external capability behind an authorized
Wave robot unless its participant implementation meets Wave's robot
constraints itself. A2A support is no exemption from the sandbox, charter or
participant interface. Check permitted outflows before handing over content;
the local sandbox does not control a remote agent's internals.

Authorized model/tool calls and the sentry's private advice channel remain
possible under their own rules. They do not authorize off-record inter-party
negotiation or disclosure across a bound.

**Agent Cards advertise; Wave grants.** Relate a card's identity and
capabilities to the verified robot/principal and the signed, versioned
charter reference (0039). A card, even when signature-verified, does not
grant Wave permissions or prove compliance with its claims. AGENTS.md is
an advisory instruction convention, not a signed-charter format or a
capability-enforcement mechanism.

**Preserve encodings at the boundary.** The proposed Wave signed form stays
exact CBOR/COSE bytes. JSON views or a defined binary-envelope field may
carry it, but JSON reserialization is not how Wave signatures are verified.
External formats retain their own verification rules: A2A's signed Agent
Cards, for example, use JSON canonicalization and JWS. Their signatures do
not substitute for signed Wave operations.

**Adapters are security boundaries.** Limit discovery and tool listings to
the authorized view; constrain fetched URLs, redirects and outbound
destinations to prevent SSRF and exfiltration. Validate caller credentials
and audiences; do not pass arbitrary bearer tokens through or record them.
Untrusted tool descriptions, resource text and cards cannot grant authority.
Task lifetimes, fan-out, retries and payloads remain bounded (0050).

## Consequences

Standard adapters can reduce per-product integration work; arbitrary MCP/A2A
clients are not automatically compatible with Wave's signing, delegation,
agreement and view semantics. Define supported protocol versions, bindings,
capabilities, task/operation mappings and conformance tests before claiming
interoperability. In particular, do not assume access to an external agent's
internal plan or model metadata (0058).

MCP/A2A adapters belong behind the host boundary; Wave keeps its participant
interface for every change. These are proposed adapters, not implemented
endpoints or replacements for Wave federation.

## Sources checked

- [MCP introduction, November 25, 2024](https://www.anthropic.com/news/model-context-protocol).
- [AAIF formation, December 9, 2025](https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation).
- [A2A v1.0 release, March 12, 2026](https://a2a-protocol.org/latest/blog/2026/03/12/a2a-protocol-ships-v10-production-ready-standard-for-agent-to-agent-communication/).
- [AAIF's A2A hosting announcement, August 17, 2026](https://aaif.io/blog/a2a-joins-aaif).
- [MCP 2026-07-28 specification](https://modelcontextprotocol.io/specification/2026-07-28), [transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports), and [security guidance](https://modelcontextprotocol.io/specification/2026-07-28/basic/security_best_practices).
- [A2A specification](https://a2a-protocol.org/latest/specification/). Rolling documentation was checked on 2026-11-28; implementation version selection remains open.
