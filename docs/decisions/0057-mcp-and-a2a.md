# 0057. MCP and A2A meet Wave at its boundary

- Status: Proposed
- Date: 2026-11-27

## Context

Two open standards now govern how AI agents connect. The Model Context
Protocol (MCP) connects an agent to tools and data. The Agent2Agent protocol
(A2A, version 1.0 in April 2026) connects agents to one another, with Agent
Cards for discovery. Both are hosted by the Agentic AI Foundation, under the
Linux Foundation, alongside AGENTS.md. Both use JSON.

The discussion paper (Appendix B) already lists A2A and MCP interactions
among the things an agent's actions must account for.

## Decision

Proposed:

**MCP is a door for agents.** A Wave server may expose its Waves through MCP:
read a wavelet, query history, propose an operation. MCP is only transport.
Everything it carries is subject to the same permissions, views and bounds
as any other door, and every change is a signed operation admitted in the
ordinary way (0005, 0010).

**A2A agents participate in Waves; they do not negotiate around them.** Agents
representing different parties coordinate inside the Wave, where it is
recorded, not over a private A2A channel. A2A messages that enter a Wave
become signed operations by robot participants acting for their principals.
If parties' agents negotiated off the record, the record would lose exactly
what a court most needs. The one sanctioned off-record channel remains a
sentry's channel to its own person.

**Agent Cards and charters correspond.** A robot's A2A Agent Card and its
published charter (0039) describe the same robot; the charter remains what
the record refers to. AGENTS.md is one of the formats a charter translates
into.

**The encodings meet at the boundary.** MCP and A2A speak JSON; Wave's signed
form stays CBOR. The JSON view (0016) is where they meet; nothing is signed or
verified from JSON.

## Consequences

Any MCP-capable or A2A-capable agent can work with Wave without a custom
integration, and gains no privilege by doing so.

A Wave MCP server and A2A participant interface are host components, behind
the host boundary, using the participant interface for every change.
