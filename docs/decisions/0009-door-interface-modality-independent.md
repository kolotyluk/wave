# 0009. The door interface is independent of how the person communicates

- Status: Accepted
- Date: 2026-11-16

## Context

The first door is a browser door. Voice, telephone and text-message doors
are expected later.

## Decision

The door interface carries what the person means and what Wave answers. It
never assumes a screen.

## Consequences

Later doors fit the same interface without changing it.

A door without a screen also has no passkey prompt, which the identity rules
must allow for (see 0021).
