# 0018. Asking is separate from changing; asking uses HTTP QUERY

- Status: Accepted
- Date: 2026-11-16

## Context

A request that only reads must never be able to alter the record.

## Decision

Anything that changes a Wave is a signed operation. A request that only asks
can never alter the record.

Wherever the protocol uses HTTP, a request that reads and needs a structured
body (a range of history, a search, an authorised semantic query) uses the
QUERY method (RFC 10008): safe, idempotent, cacheable and repeatable, with the
query kept out of the address.

## Consequences

Support for QUERY in libraries, proxies and gateways is to be tested early.

QUERY is not a CORS-safelisted method, so a cross-origin QUERY from the
browser triggers a preflight request.

OpenAPI 3.2 describes QUERY natively (see 0023).
