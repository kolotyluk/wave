# 0042. IPv6 first

- Status: Proposed
- Date: 2026-11-20

## Context

IPv6 gives every host its own address, with no address translation in
between. Major Canadian ISPs and clouds support it. Some networks people use
are still IPv4-only. AWS charges for every public IPv4 address.

## Decision

Proposed: **Wave is built and tested on IPv6 first.**

- **The IP version is the host's business**, as the HTTP version is (0041).
  Wave's code never depends on it.
- **The basement: IPv4**, by dual-stacking the host. Required before any public
  deployment.
- **With no address translation, the firewall is the boundary.** Only the
  ports Wave needs (443 over UDP and TCP) are open, and only to Wave hosts.

## Consequences

Development and federation testing need no address translation, and the
public demonstration needs no paid IPv4 address until the basement is built.
