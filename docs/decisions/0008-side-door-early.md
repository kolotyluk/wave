# 0008. The Side Door is built early, in a minimal form

- Status: Accepted
- Date: 2026-11-16

## Context

The paper describes a Side Door: a direct view of structure, permissions,
agents and history for those who need direct control. Testing needs the same
view from the start.

## Decision

A minimal Side Door, `wave-side`, is built early, read-only at first, as the
way to inspect the record.

## Consequences

The record can be inspected by eye from step 3 of the order of work onward.

A second door from the start keeps the door interface honest.

Editing through the Side Door, and making it fit for the public, are deferred.
