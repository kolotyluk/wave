# 0024. The live stream starts on WebSocket, with WebTransport to be tested

- Status: Proposed
- Date: 2026-11-19

## Context

Live editing needs a long-lived, two-way channel. WebSocket works everywhere
and is final. WebTransport over HTTP/3 is in every major browser since Safari
26.4 (March 2026), but its protocol is still an Internet-Draft, and some
networks block the UDP it runs on.

## Decision

Proposed: the stream is built on WebSocket, carrying CBOR in binary frames.
The stream layer is one Rust trait, so that WebTransport can be added as the
preferred route, chosen at runtime by what the browser offers and what
connects.

WebSocket and WebTransport are tested side by side, including fallback when
WebTransport is blocked.

## Consequences

Operations are self-contained signed objects, so the transport can change
without touching the protocol.

Ordinary HTTP requests need no decision: the server offers HTTP/3 and HTTP/2,
and the browser chooses.

Whether the server's WebAssembly runtime can handle QUIC at all is part of the
runtime proof (0026).
