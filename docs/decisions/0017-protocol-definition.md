# 0017. Rust is the working protocol definition; test vectors are the neutral truth

- Status: Accepted
- Date: 2026-11-16

## Context

Wave must be implementable in any language, but there is only one
implementation so far.

## Decision

For now the protocol lives in `wave-protocol` and nowhere else. The crate holds
only the shapes of messages and operations and the rules for encoding them:
no input or output, storage, networking or interface.

The encoding is specified deliberately and never left to a library's defaults.

The `vectors/` directory holds example messages with their expected bytes and
results. They are not Rust. Any implementation must pass them.

A written specification follows when a second implementation is attempted.

## Consequences

The move to a written specification stays cheap.

The test vectors are the start of the conformance suite.
