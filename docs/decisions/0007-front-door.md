# 0007. The Front Door is its own pluggable component

- Status: Accepted
- Date: 2026-11-16

## Context

Most people should never have to learn Wave to use it.

## Decision

The Front Door is its own component, `wave-door`. It stands between the
person and Wave: plain language in, visible structure out. It is pluggable,
with a very decent default.

## Consequences

Others can supply a different door without touching the record.

The door interface is defined separately from any one door (see 0009).
