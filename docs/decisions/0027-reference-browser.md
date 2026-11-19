# 0027. Firefox is the reference browser

- Status: Proposed
- Date: 2026-11-19

## Context

Development needs one browser to work against. Firefox's engine, Gecko, is the
only major browser engine not controlled by Apple or Google.

## Decision

Proposed: Firefox is the reference browser for development. All three engines
(Gecko, Blink and WebKit) are tested before anything ships.

## Consequences

On iOS every browser uses WebKit, so WebKit support is not optional.
