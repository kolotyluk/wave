# 0014. Invent only the model; standardize everything else

- Status: Accepted
- Date: 2026-11-19

## Context

What aged worst in Apache Wave was everything it built for itself beneath the
model.

## Decision

What Wave adds to the world is the shared object, its operations, and the
rules for admission and federation. Everything beneath that uses an existing,
stable standard.

## Consequences

Each layer below the model is a choice among standards, recorded here:
encoding and signing (0015), media type (0016), HTTP (0018, 0023), transport
(0024), identity frameworks (0021), time-stamping and evidence (0022).
