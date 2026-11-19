# 0013. Apache Wave is lineage and reference, not runtime

- Status: Accepted
- Date: 2026-11-16

## Context

Apache Wave was retired in 2018. Its model and its specifications remain
sound. Its plumbing (its own RPC, a protobuf-to-JSON mapping for the browser,
federation over XMPP) has aged badly. The XMPP federation code was removed
in 2016 and survives only in the history.

## Decision

Apache Wave's model and protocol documents are the inheritance. Its Java code
is something to test against, not to run, and no Apache Wave code or document
is copied into this repository.

If a working copy is needed, the archive is forked or cloned separately, from
`apache/incubator-retired-wave` on GitHub. Its specifications and whitepapers
are found at the release tags (`wave-0.4-rc1` and later), not on the main line.

## Consequences

The repository stays clean. The archive stays available.
