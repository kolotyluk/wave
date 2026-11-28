# 0056. AI-ready views are derived; tokens are never stored

- Status: Proposed
- Date: 2026-11-27

## Context

AI agents read Waves constantly: the Front Door agent, sentries, intrinsic
robots, retrieval for search. It is tempting to store material in a form
models consume directly, such as pre-computed tokens.

Tokenization is specific to each model family and often changes between
versions. It is cheap compared with everything else a model does. A record
meant to last decades must not depend on one vendor's current model.

## Decision

Proposed:

**Tokens are never stored in the record.** The record stays model-neutral:
signed CBOR operations and blobs (0043).

**What helps AI is structure, anchors and derived views.**

- **Structure.** A model works best with text that knows who said what, when,
  in reply to what, and under which bound. The Wave document model already
  carries this; the Wave DOM (0058) exposes it.
- **Stable anchors.** Every passage can be cited by operation identifier, so
  an agent's answers point to the exact record rather than paraphrasing it.
- **Derived views**, held as rebuildable state (0043, 0045):
  - a text-first rendering of each wavelet, with speakers, times and anchors;
  - text extracted from attachments, with its source blob's hash;
  - chunked text and embeddings for retrieval;
  - summaries.

  Each records the model and version that produced it, its chunking or format
  version, and the point in the log it was derived from. Each inherits the
  bounds and views of its source: a summary of without-prejudice content is
  itself without prejudice (0037). Sending content to an outside model to
  derive a view is an outflow and needs authorization.

**Prompt caching comes almost for free.** AI providers cache a prompt's
repeated beginning. Because the log is append-only, a wavelet's context
mostly grows at the end and its earlier part stays identical between
requests. Renderings keep a stable order so that this holds.

## Consequences

Changing models means rebuilding derived views, never rewriting the record.

Derived views count against resource limits (0050).
