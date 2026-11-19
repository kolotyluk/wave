# 0023. The API is HTTP with CBOR bodies, not gRPC

- Status: Accepted
- Date: 2026-11-19

## Context

Apache Wave used its own RPC. gRPC is the common modern alternative, but
browsers cannot speak it natively, and it hides the HTTP method.

## Decision

Wave's API is an HTTP API with CBOR bodies. It follows HTTP's semantics
rather than REST purity: resources where they fit, QUERY for asking, and
submissions for signed operations.

Endpoints are to be described in OpenAPI 3.2, in `spec/`, with CDDL as the
authority for bodies. Live traffic is a stream, not a request (see 0024).

All endpoints are drafts, visible for debate, until the identifiers (0028),
the signed-operation envelope (0029) and the CRDT library (0025) are settled.

## Consequences

Signed, hash-chained history entries never change, so each can carry its hash
as an ETag and be cached forever.

Inspecting traffic needs a CBOR decoder; diagnostic notation and the JSON view
cover that.

If server-to-server federation ever wants gRPC, it can carry CBOR without
changing the encoding.
