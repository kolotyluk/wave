# 0041. HTTP/3 is the ground floor

- Status: Proposed
- Date: 2026-11-20
- Supersedes: [0024](0024-live-stream-transport.md)

## Context

HTTP/3 runs over QUIC, which is built from independent streams, survives a
change of network, and carries WebTransport. Some networks block the UDP it
runs on, mostly at the edge: corporate firewalls, public Wi-Fi, institutional
networks. Browsers usually discover HTTP/3 from an earlier HTTP/2 response or
from a DNS HTTPS record (RFC 9460).

WebSocket over HTTP/3 is standardized (RFC 9220) but thinly supported in
browsers.

## Decision

Proposed: **HTTP/3 is the ground floor**, as WebAssembly is (0002).

- **The skeleton and development run on HTTP/3.**
- **The HTTP version is the host's business.** The Wave server component does
  not open sockets. It handles requests through WebAssembly's standard HTTP
  interface (wasi-http); the runtime hosting it terminates QUIC, TLS and
  HTTP/3 (0026). Wave's code never depends on the HTTP version.
- **The live stream is WebTransport** over HTTP/3. Operations are
  self-contained signed objects, so the stream carries them without depending
  on the transport.
- **HTTP/3 is advertised in DNS** with HTTPS records, so that browsers can use
  it from the first connection.
- **The basement: HTTP/2, with the stream over WebSocket.** Required before any
  public deployment, because some people's networks block UDP. Because the
  host handles the HTTP version, the basement is a configuration of the host,
  not a change to Wave.

## Consequences

WebTransport's protocol is still an Internet-Draft; the skeleton accepts that.

Some browsers consult DNS HTTPS records only with secure DNS (DNS over HTTPS)
enabled. The development environment documents the reference browser's
settings, with a browser flag as fallback.

Networks that block UDP are mostly run or funded by someone who can choose
otherwise. Asking governments to allow HTTP/3 on networks they run or fund
shrinks the problem but does not remove it; the basement stays required.
