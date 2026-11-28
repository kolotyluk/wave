# 0058. A Wave DOM serves people and agents alike; agent metadata is recorded

- Status: Proposed
- Date: 2026-11-27

## Context

Google Wave modelled XML documents with independent range annotations (see
the [comparison's sources](../google-wave-and-canada-wave.md#sources)).
Browsers expose an accessibility tree derived from content and semantics
for assistive technology. That is useful inspiration for an agent-readable
projection, not a claim that Wave's record is literally a browser DOM or
that every browser node appears in an accessibility tree.

The paper's Appendix B requires that an agent's actions record its model and
runtime version, plan or rationale, tools and resources used, A2A and MCP
interactions, and sources consulted.

## Decision

Proposed:

**The Wave DOM is a semantic projection of the record.** A tree with explicit
references may present documents and their relationships; it does not
replace the causal history with a tree.

- **Semantic roles** on nodes: statement, question, claim, deadline,
  decision, exhibit, sworn statement, dispute, and others. Roles are stand-off
  annotations, drawn where possible from standard vocabularies (optional
  JSON-LD contexts, as the paper allows). Record the annotation's author,
  basis and version; generated labels are interpretations. Calling a node
  "sworn statement" or "decision" cannot confer legal status or authority.
- **Stable identifiers and versioned selectors:** document/node identity,
  source checkpoint/version, and passage ranges as appropriate (0056).
  Operation IDs establish contributing provenance but are not interchangeable
  with node IDs; one operation may affect several nodes or ranges. Exact
  identity/selector rules remain open under 0028/0029 and the CRDT choice.
- **Provenance and visibility** on every node: who, when, acting for whom,
  under which bound. Distinguish participant assertions, server observations
  and independent attestations, without disclosing protected root links.
- **Two serializations:** a compact text form for models, and the full
  authorized structure for the Side Door. Both are constrained by the
  caller's permitted view; a model renderer is not a way to access hidden
  source material.

**Shared semantics support accessibility and agents.** This is not a
substitute for an accessible interface: keyboard/focus behavior, screen
reader navigation, alternatives to non-text content and meaningful status
announcements still need design and testing with people. Model-oriented
compression must not remove information an authorized person needs.

**Agent metadata is recorded, not shown by default, never hidden.** With each
agent act, the proposal is to record a bounded account of the available
model/runtime identifiers, charter (0039), declared intent or concise
decision summary, tool/resource use, A2A/MCP interactions, and source
references. Distinguish what Wave observed from what an external provider
declared and what is unavailable. A2A does not require disclosure of an
agent's private internals; do not demand or invent hidden chain-of-thought.

Metadata is collapsed by default and inspectable by viewers entitled to it,
like intrinsic robots (0048). "Never hidden" is not permission to expose
protected metadata, its existence or its size to unauthorized viewers.

- **It is data, never instructions.** Text that people do not normally see but
  agents read is a classic channel for prompt injection. Agents treat all
  retrieved metadata as untrusted content, not as a new charter or principal
  instruction. The Side Door exposes the authorized view. Charter guidance
  alone is not a security boundary: capability, authorization and outflow
  checks remain enforced independently of model behavior.
- **It inherits bounds and views.** A tool call or source list can reveal
  protected content. Apply the restrictions of every source it reveals, not
  merely the destination wavelet's view.
- **It never holds secrets**: no private keys or authentication credentials.
  It is part of an append-only record. Allowlist safe fields before
  admission; exclude bearer tokens, signed access URLs, passwords and raw
  credential-bearing dumps from referenced trace blobs too. Record omissions
  where appropriate without leaking the omitted information.
- **It is bounded.** Full traces may be stored as blobs referenced by hash,
  with a summary in the record only when their safe contents and disclosure
  are authorized. A content hash or encryption does not by itself authorize
  collecting or retaining a trace. Sizes, retention, and any erasure design
  follow the same limits as other evidence and derivatives (0050, 0056).

**Completion does not rewrite an act.** Bind the metadata known at admission
to the signed act, then append linked completion/failure attestations as
later information arrives. Exact envelope and trace-link rules remain open
(0029). Distinguish recorded intent, attempted tool execution, observed
result and verified external effect; do not report an attempt as success.

**Substantive inter-party communications are content, not hidden metadata.**
Negotiation and other material contributions are ordinary signed operations
under their applicable views (0057). Discovery and task-progress bookkeeping
are not automatically negotiation content. Metadata describes the *how*
behind an agent's act, not a place to conceal its substance.

## Consequences

An authorized person or auditor can inspect the recorded account of an
agent's sources and actions, including its gaps. A signature authenticates
that account's attribution and integrity, not the truth of a model's
explanation or completeness of an opaque provider's internal history.

The Wave DOM replaces neither the record nor the CRDT; it is a derived view
(0056), rebuilt from the log.

Before implementation, specify metadata fields and producers, missing-data
semantics, signed binding, selector stability and projection versions.
Test access control, secret exclusion, prompt injection, late results,
replay, and accessibility; none is established merely by naming a DOM.
