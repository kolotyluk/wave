# 0056. AI-ready views are derived; the record stays model-neutral

- Status: Proposed
- Date: 2026-11-27

## Context

AI agents read Waves constantly: the Front Door agent, sentries, intrinsic
robots, retrieval for search. It is tempting to store material in a form
models consume directly, such as pre-computed tokens.

Tokenization is specific to a tokenizer and its version; models may share
one, but compatibility must not be assumed. A record meant to last decades
must not depend on one vendor's current model or inference cache.

## Decision

Proposed:

**Model token IDs are not authoritative record content.** Preserve the exact
model-neutral signed operations and source blobs (0043), not tokenized
substitutes. This does not prohibit bounded, disposable host-side tokenizer
or inference caches. Authentication tokens and credentials are a different
matter: they never belong in the record or its trace blobs (0058).

**What helps AI is structure, anchors and derived views.**

- **Structure.** Render attributed content, reply relationships, applicable
  bounds, and source progress explicitly. Distinguish participant assertions
  from server-observed times and independently witnessed checkpoints. Exact
  fields depend on the unresolved envelope/document models; the proposed
  Wave DOM (0058) exposes them without inventing missing provenance.
- **Stable anchors.** An operation ID alone does not identify every node or
  passage it affects. A citation needs the document/node identity, source
  version or checkpoint, and an appropriate range/selector, backed by the
  relevant operations. Attachment references also need a source blob hash
  and page, region or equivalent locator. Persistent selectors and behavior
  after concurrent edits, redaction or deletion remain to be specified.
- **Derived views**, held as rebuildable state (0043, 0045):
  - a text-first rendering of each wavelet, with speakers, times and anchors;
  - text extracted from attachments, with its source blob's hash;
  - chunked text and embeddings for retrieval;
  - summaries.

  Each records its source hashes and scoped checkpoint/cursors, producer
  and format version, and, where applicable, model/tokenizer version,
  chunking, and embedding dimensions. Record changes and omissions honestly;
  OCR, summaries and inferred labels are interpretations, not source facts.
  Each inherits applicable restrictions from all its sources: a summary of
  without-prejudice content remains restricted (0037). A combined view must
  not adopt a less restrictive source's permissions.

**Derived state is also a disclosure surface.** Check authorization before
returning results, snippets, scores, counts or summaries. Isolate indexes
and caches by the relevant scope/view/policy, invalidate or rebuild stale
material after permission or content changes, and define retention and
erasure for every derivative. Rebuildability does not make plaintext safe
to retain. Sending content to an outside model, including its caches and
logs, is an outflow requiring the appropriate authorization.

**Prompt caching is an optimization, not a log property.** Some providers
reuse an identical rendered prefix, subject to their model, cache format,
minimum size, configuration, retention and pricing rules. An append-only
operation log does not imply an append-only prompt: edits, view changes,
compaction and instruction/tool changes can alter earlier rendered text.
Stable rendering of unchanged authorized content can help, but cache hits
must be measured, never promised or relied on for correctness.

Never preserve an obsolete disclosure grant to improve cache reuse.
Provider-side retention and deletion guarantees must be evaluated separately
from local invalidation; neither recalls plaintext already disclosed.

**Publishing a derivative is an act.** A disposable summary cache is
rebuildable; a summary or extracted result published as a signed operation
is also evidence of what that participant actually said. Preserve that
act's exact bytes and provenance. Regeneration creates a new result or
correction, not a silent rewrite of published evidence.

## Consequences

Changing a model or renderer rebuilds the affected derivatives, never the
record; unrelated derivatives need not be regenerated.

Derived views count against resource limits (0050).

Before implementation, specify citation selectors, source/policy versions,
cache isolation/invalidation, and tests for stale or unauthorized retrieval.

## Reference

[OpenAI's prompt-caching guide](https://platform.openai.com/docs/guides/prompt-caching)
illustrates provider-specific prefix matching; it is not a portable Wave
guarantee or a choice of provider.
