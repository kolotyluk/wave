# 0058. A Wave DOM serves people and agents alike; agent metadata is recorded

- Status: Proposed
- Date: 2026-11-27

## Context

Google Wave's document was a DOM in all but name (discussion paper, Appendix
G). Browsers maintain a second tree beside the DOM: the accessibility tree,
which says what each element is rather than how it is drawn. Screen readers
use it, and increasingly so do AI agents, because it is compact and
meaningful.

The paper's Appendix B requires that an agent's actions record its model and
runtime version, plan or rationale, tools and resources used, A2A and MCP
interactions, and sources consulted.

## Decision

Proposed:

**The Wave DOM is a tree of the record, designed like an accessibility tree.**

- **Semantic roles** on nodes: statement, question, claim, deadline,
  decision, exhibit, sworn statement, dispute, and others. Roles are stand-off
  annotations, drawn where possible from standard vocabularies (optional
  JSON-LD contexts, as the paper allows).
- **Stable identifiers** on every node: operation identifiers, so anything can
  be cited exactly.
- **Provenance and visibility** on every node: who, when, acting for whom,
  under which bound.
- **Two serializations:** a compact text form for models, and the full
  structure for the Side Door.

**One model serves blind people and AI agents together.** Both need meaning
without visual noise. Accessibility is not a separate effort; it is the
foundation.

**Agent metadata is recorded, not shown by default, never hidden.** With each
agent act, Wave records the metadata the paper requires: model and runtime
version, charter (0039), plan or rationale, tools and resources used, A2A and
MCP interactions, and sources consulted. It is collapsed by default and shown
in full on request, like intrinsic robots (0048).

- **It is data, never instructions.** Text that people do not normally see but
  agents read is a classic channel for prompt injection. Agents treat all
  metadata as data (charter rule 2), and the Side Door always shows it in
  full.
- **It inherits bounds and views.** A tool call or source list can reveal
  protected content, so metadata is protected like what it describes.
- **It never holds secrets**: no keys, tokens or credentials. It is part of an
  append-only record.
- **It is bounded.** Full traces may be stored as blobs referenced by hash,
  with a summary in the record; they count against resource limits (0050).

**Messages between agents representing different parties are content, not
metadata.** They are the negotiation itself, recorded as ordinary signed
operations and as visible as anything else (0057). Metadata is the *how*
behind an agent's act.

## Consequences

A court, a person or an auditor can ask what an agent consulted before it
said something, and get an answer.

The Wave DOM replaces neither the record nor the CRDT; it is a derived view
(0056), rebuilt from the log.
